mod dispatcher;
mod transformers;
mod util;

use crate::app::Katalyst;
use crate::expression::*;
use crate::modules::*;
use crate::prelude::*;
use futures::future::*;
use futures::Future;
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

#[derive(Debug)]
pub struct HostModule {}

impl Module for HostModule {
    fn name(&self) -> &'static str {
        "host"
    }

    fn module_type(&self) -> ModuleType {
        ModuleType::RequestHandler
    }

    fn build(
        &self,
        engine: Arc<Katalyst>,
        config: &ModuleConfigLoader,
    ) -> Result<Arc<ModuleDispatch>, ConfigurationFailure> {
        let c: HostConfig = config.load()?;
        let providers = engine.get_compiler();
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
        Ok(Arc::new(HostDispatcher {
            host: c.host.to_owned(),
            path: providers.compile_template(Some(c.path.as_str()))?,
            method,
            query: providers.compile_template_map(&c.query)?,
            headers: providers.compile_template_map(&c.headers)?,
            body: providers.compile_template_option(body)?,
        }))
    }
}

impl ModuleDispatch for HostDispatcher {
    fn dispatch(&self, ctx: Context) -> ModuleResult {
        Box::new(
            result(self.prepare(ctx))
                .and_then(HostDispatcher::send)
                .map(HostDispatcher::clean_response),
        )
    }
}

impl HostDispatcher {
    pub fn transformer(
        &self,
        ctx: &Context,
        lease_str: String,
    ) -> Result<DownstreamTransformer, RequestFailure> {
        let mut uri = lease_str;
        uri.push_str(&self.path.render(ctx)?);
        if let Some(query) = &self.query {
            uri.push_str("?");
            for (key, val) in query.iter() {
                uri.push_str(&key);
                uri.push_str("=");
                uri.push_str(&val.render(&ctx)?);
                uri.push_str("&");
            }
            uri.truncate(uri.len() - 1);
        };

        let method = self.method.clone();

        let headers = match &self.headers {
            Some(h) => Some(
                h.iter()
                    .map(|(key, val)| Ok((key.to_string(), val.render(ctx)?)))
                    .collect::<Result<HashMap<String, String>, RequestFailure>>()?,
            ),
            None => None,
        };

        let body = match &self.body {
            Some(b) => Some(b.render(&ctx)?),
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
