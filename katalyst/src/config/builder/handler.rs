use super::Builder;
use crate::app::KatalystEngine;
use crate::error::ConfigurationFailure;
use crate::expression::Compiler;
use crate::instance::Handler;
use crate::instance::HostDispatcher;
use http::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::string::String;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum HandlerBuilder {
    Host {
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
    },
    Content {
        path: String,
    },
}

impl Default for HandlerBuilder {
    fn default() -> Self {
        HandlerBuilder::Host {
            host: String::default(),
            path: String::default(),
            method: None,
            query: None,
            headers: None,
            body: None,
        }
    }
}

impl Builder<Handler> for HandlerBuilder {
    fn build(&self, engine: Arc<KatalystEngine>) -> Result<Handler, ConfigurationFailure> {
        match self {
            HandlerBuilder::Host {
                host,
                path,
                method,
                query,
                headers,
                body,
            } => {
                let providers = engine.locate::<Compiler>()?;
                let method = match method {
                    Some(m) => Some(Method::from_bytes(m.to_uppercase().as_bytes())?),
                    None => None,
                };
                let body = match body {
                    Some(bod) => Some(bod.as_str()),
                    None => None,
                };
                Ok(Handler::Host(HostDispatcher {
                    host: host.to_owned(),
                    path: providers.compile_template(Some(path.as_str()))?,
                    method,
                    query: providers.compile_template_map(query)?,
                    headers: providers.compile_template_map(headers)?,
                    body: providers.compile_template_option(body)?,
                }))
            }
            HandlerBuilder::Content { path: _ } => Err(ConfigurationFailure::InvalidResource),
        }
    }
}
