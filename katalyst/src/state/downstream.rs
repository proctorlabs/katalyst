use crate::error::KatalystError;
use crate::expression::*;
use crate::prelude::*;
use http::header::{HeaderName, HeaderValue};
use http::Method;
use hyper::Body;
use hyper::Request;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct Downstream {
    pub host: String,
    pub path: Expression,
    pub method: Option<Method>,
    pub query: Option<HashMap<String, Expression>>,
    pub headers: Option<HashMap<String, Expression>>,
    pub body: Option<Expression>,
}

impl Downstream {
    pub fn transformer(
        &self,
        ctx: &Context,
        lease_str: String,
    ) -> Result<DownstreamTransformer, KatalystError> {
        let mut uri = lease_str;
        uri.push_str(&self.path.render(ctx));
        if let Some(query) = &self.query {
            uri.push_str("?");
            for (key, val) in query.iter() {
                uri.push_str(&key);
                uri.push_str("=");
                uri.push_str(&val.render(&ctx));
                uri.push_str("&");
            }
            uri.truncate(uri.len() - 1);
        };

        let method = self.method.clone();

        let headers = match &self.headers {
            Some(h) => Some(
                h.iter()
                    .map(|(key, val)| (key.to_string(), val.render(ctx)))
                    .collect(),
            ),
            None => None,
        };

        let body = match &self.body {
            Some(b) => Some(b.render(&ctx)),
            None => None,
        };

        Ok(DownstreamTransformer {
            uri,
            method,
            headers,
            body,
        })
    }
}

#[derive(Debug)]
pub struct DownstreamTransformer {
    pub uri: String,
    pub method: Option<Method>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

impl DownstreamTransformer {
    pub fn transform(self, req: Request<Body>) -> Result<Request<Body>, RequestFailure> {
        let (mut parts, mut body) = req.into_parts();
        parts.uri = self.uri.parse()?;

        if let Some(method) = self.method {
            parts.method = method;
        }

        if let Some(body_str) = self.body {
            body = hyper::Body::from(body_str);
        }

        if let Some(headers) = self.headers {
            for (key_str, val_str) in headers.iter() {
                if let (Ok(key), Ok(val)) = (
                    HeaderName::from_str(&key_str),
                    HeaderValue::from_str(val_str),
                ) {
                    while parts.headers.contains_key(key_str) {
                        parts.headers.remove(key_str);
                    }
                    parts.headers.append(key, val);
                }
            }
        }

        Ok(Request::from_parts(parts, body))
    }
}
