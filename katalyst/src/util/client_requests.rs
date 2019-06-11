use crate::{app::HttpsClient, config::builder::Builder, expression::*, prelude::*};
use futures::{future::*, Future};
use http::{
    header::{HeaderMap, HeaderName, HeaderValue},
    request::Builder as RequestBuilder,
    Method, Request, Response, StatusCode,
};
use hyper::Body;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr, sync::Arc};
use unstructured::Document;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct ClientRequestBuilder {
    pub host: String,
    pub path: String,
    pub method: Option<String>,
    pub query: Option<HashMap<String, String>>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

impl Builder<CompiledClientRequest> for ClientRequestBuilder {
    fn build(&self, katalyst: Arc<Katalyst>) -> Result<CompiledClientRequest> {
        let compiler = katalyst.get_compiler();

        let method = match &self.method {
            Some(m) => Some(Method::from_bytes(m.to_uppercase().as_bytes())?),
            None => None,
        };

        let temp;
        let body = match &self.body {
            Some(bod) => {
                temp = bod;
                Some(temp.as_str())
            }
            None => None,
        };

        Ok(CompiledClientRequest {
            host: self.host.to_owned(),
            path: compiler.compile_template(Some(self.path.as_str()))?,
            method,
            query: compiler.compile_template_map(&self.query)?,
            headers: compiler.compile_template_map(&self.headers)?,
            body: compiler.compile_template_option(body)?,
        })
    }
}

#[derive(Debug)]
pub struct CompiledClientRequest {
    host: String,
    path: Expression,
    method: Option<Method>,
    query: Option<HashMap<String, Expression>>,
    headers: Option<HashMap<String, Expression>>,
    body: Option<Expression>,
}

impl CompiledClientRequest {
    pub fn prepare_request(&self, ctx: &RequestContext) -> Result<HttpData> {
        let mut path = self.path.render(ctx)?;

        if let Some(query) = &self.query {
            let raw_query = query
                .iter()
                .map(|(k, v)| Ok(format!("{}={}", k, v.render(ctx)?)))
                .collect::<Result<Vec<String>>>()?
                .join("&");
            path = format!("{}?{}", path, raw_query);
        }

        let headers = if let Some(hdrs) = &self.headers {
            hdrs.iter()
                .map(|(k, v)| {
                    Ok((
                        HeaderName::from_str(k).unwrap(),
                        HeaderValue::from_str(&v.render(ctx)?).unwrap(),
                    ))
                })
                .collect::<Result<HeaderMap<HeaderValue>>>()?
        } else {
            HeaderMap::default()
        };

        let host = ctx
            .katalyst()?
            .get_instance()?
            .hosts
            .get(&self.host)
            .ok_or_else(|| fail!(_ NOT_FOUND))?
            .servers
            .lease()?;

        let body =
            if let Some(body) = &self.body { Body::from(body.render(ctx)?) } else { Body::empty() };

        Ok(HttpData {
            request_type: HttpAction::Request(
                format!("{}{}", host, path),
                self.method.as_ref().cloned().unwrap_or(Method::GET),
            ),
            headers,
            body: HttpContent::Raw(body),
        })
    }
}

#[derive(Debug)]
pub enum HttpContent {
    Raw(Body),
    Bytes(Vec<u8>),
    Parsed(Document),
}

impl HttpContent {
    pub fn into_body(self) -> Body {
        match self {
            HttpContent::Raw(body) => body,
            HttpContent::Bytes(bytes) => Body::from(bytes),
            HttpContent::Parsed(doc) => Body::from(serde_json::to_vec(&doc).unwrap()),
        }
    }
}

#[derive(Debug)]
pub enum HttpAction {
    Request(String, Method),
    Response(StatusCode),
}

#[derive(Debug)]
pub struct HttpData {
    pub request_type: HttpAction,
    pub headers: HeaderMap<HeaderValue>,
    pub body: HttpContent,
}

impl From<Request<Body>> for HttpData {
    fn from(req: Request<Body>) -> HttpData {
        let (parts, body) = req.into_parts();
        HttpData {
            request_type: HttpAction::Request(parts.uri.to_string(), parts.method),
            headers: parts.headers,
            body: HttpContent::Raw(body),
        }
    }
}

impl From<Response<Body>> for HttpData {
    fn from(req: Response<Body>) -> HttpData {
        let (parts, body) = req.into_parts();
        HttpData {
            request_type: HttpAction::Response(parts.status),
            headers: parts.headers,
            body: HttpContent::Raw(body),
        }
    }
}

impl HttpData {
    pub fn send(self, client: &HttpsClient) -> AsyncResult<HttpData> {
        if let HttpAction::Request(uri, method) = self.request_type {
            let mut request = RequestBuilder::new();
            request.method(method);
            request.uri(&uri);
            *request.headers_mut().unwrap() = self.headers;
            let req = request.body(self.body.into_body()).unwrap();
            let res = client.request(req);
            Box::new(res.then(move |response| match response {
                Ok(r) => ok::<HttpData, GatewayError>(HttpData::from(r)),
                Err(e) => {
                    err(fail!(_ GATEWAY_TIMEOUT, format!("Error sending request to {}", uri), e))
                }
            }))
        } else {
            Box::new(err::<HttpData, GatewayError>(err!(Other, "Response type cannot be sent")))
        }
    }

    pub fn send_parse(self, client: &HttpsClient) -> AsyncResult<HttpData> {
        Box::new(self.send(client).and_then(|mut resp| {
            let content_type = resp.headers.get("Content-Type").map(|m| m.to_str().unwrap());
            let format = Format::content_type(content_type);
            if let HttpContent::Raw(body) = resp.body {
                resp.body = HttpContent::Bytes(vec![]);
                return Box::new(Either::A(body.concat2().then(move |r| {
                    let doc = match r {
                        Ok(d) => format.parse(&d).unwrap_or_default(),
                        Err(_) => Document::Unit,
                    };
                    resp.body = HttpContent::Parsed(doc);
                    Box::new(ok::<HttpData, GatewayError>(resp))
                })));
            }
            Box::new(Either::B(ok(resp)))
        }))
    }
}
