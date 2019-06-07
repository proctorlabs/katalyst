use crate::{config::builder::Builder, expression::*, prelude::*};
use http::Method;
use hyper::Body;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct ClientRequestBuilder {
    pub scheme: String,
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
            scheme: self.scheme.to_owned(),
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
    scheme: String,
    host: String,
    path: Expression,
    method: Option<Method>,
    query: Option<HashMap<String, Expression>>,
    headers: Option<HashMap<String, Expression>>,
    body: Option<Expression>,
}

impl CompiledClientRequest {
    pub fn prepare_request(&self, ctx: &RequestContext) -> Result<ClientRequest> {
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
                .map(|(k, v)| Ok((k.to_string(), v.render(ctx)?)))
                .collect::<Result<HashMap<String, String>>>()?
        } else {
            HashMap::default()
        };

        let host = ctx
            .katalyst()?
            .get_instance()?
            .hosts
            .get(&self.host)
            .ok_or_else(|| GatewayError::NotFound)?
            .servers
            .lease()?;

        let body =
            if let Some(body) = &self.body { Body::from(body.render(ctx)?) } else { Body::empty() };

        Ok(ClientRequest {
            url: format!("{}://{}/{}", self.scheme, host, path),
            method: self.method.as_ref().cloned().unwrap_or(Method::GET),
            headers,
            body,
        })
    }
}

#[derive(Debug)]
pub struct ClientRequest {
    url: String,
    method: Method,
    headers: HashMap<String, String>,
    body: Body,
}

impl ClientRequest {}
