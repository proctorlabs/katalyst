mod dispatcher;
mod transformers;
mod util;

use crate::{expression::*, modules::*};
use futures::{future::*, Future};
use http::Method;
use std::collections::HashMap;
use transformers::DownstreamTransformer;
pub use util::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct HostConfig {
    host: String,
    path: String,
    #[serde(default)]
    method: Option<String>,
    #[serde(default)]
    query: Option<HashMap<String, String>>,
    #[serde(default)]
    headers: Option<HashMap<String, String>>,
    #[serde(default)]
    body: Option<String>,
}

#[derive(Debug)]
pub struct HostDispatcher {
    pub host: String,
    pub path: Expression,
    pub method: Option<Method>,
    pub query: Option<HashMap<String, Expression>>,
    pub headers: Option<HashMap<String, Expression>>,
    pub body: Option<Expression>,
}

#[derive(Debug, Default)]
pub struct HostModule;

impl ModuleProvider for HostModule {
    fn name(&self) -> &'static str {
        "host"
    }

    fn build(&self, _: ModuleType, config: &unstructured::Document) -> Result<Module> {
        let c: HostConfig = config.clone().try_into().map_err(|e| {
            err!(ConfigurationFailure, "Failed to parse host proxy module configuration", e)
        })?;
        let method = match c.method {
            Some(m) => Some(Method::from_bytes(m.to_uppercase().as_bytes())?),
            None => None,
        };
        let temp;
        let body = match c.body {
            Some(bod) => {
                temp = bod;
                Some(temp.as_str())
            }
            None => None,
        };
        Ok(HostDispatcher {
            host: c.host.to_owned(),
            path: Compiler::compile_template(Some(c.path.as_str()))?,
            method,
            query: Compiler::compile_template_map(&c.query)?,
            headers: Compiler::compile_template_map(&c.headers)?,
            body: Compiler::compile_template_option(body)?,
        }
        .into_module())
    }
}

impl RequestHandlerModule for HostDispatcher {
    fn dispatch(&self, guard: RequestContext) -> ModuleResult {
        let guard2 = guard.clone();
        Box::new(
            result(self.prepare(guard.clone()))
                .and_then(move |_| HostDispatcher::send(guard))
                .then(move |_| HostDispatcher::clean_response(guard2)),
        )
    }
}

impl HostDispatcher {
    pub fn transformer(
        &self,
        guard: RequestContext,
        lease_str: String,
    ) -> Result<DownstreamTransformer> {
        let mut uri = lease_str;
        uri.push_str(&self.path.render(&guard)?);
        if let Some(query) = &self.query {
            uri.push_str("?");
            for (key, val) in query.iter() {
                uri.push_str(&key);
                uri.push_str("=");
                uri.push_str(&val.render(&guard)?);
                uri.push_str("&");
            }
            uri.truncate(uri.len() - 1);
        };

        let method = self.method.clone();

        let headers = match &self.headers {
            Some(h) => Some(
                h.iter()
                    .map(|(key, val)| Ok((key.to_string(), val.render(&guard)?)))
                    .collect::<Result<HashMap<String, String>>>()?,
            ),
            None => None,
        };

        let body = match &self.body {
            Some(b) => Some(b.render(&guard)?),
            None => None,
        };

        Ok(DownstreamTransformer { uri, method, headers, body })
    }
}
