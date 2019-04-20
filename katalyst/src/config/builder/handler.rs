use super::Builder;
use crate::app::KatalystEngine;
use crate::error::ConfigurationFailure;
use crate::modules::*;
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
    FileServer {
        root_path: String,
        selector: String,
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

impl Builder<Arc<ModuleDispatch>> for HandlerBuilder {
    fn build(
        &self,
        engine: Arc<KatalystEngine>,
    ) -> Result<Arc<ModuleDispatch>, ConfigurationFailure> {
        let modules: Arc<Modules> = engine.locate()?;
        match self {
            HandlerBuilder::Host { .. } => Ok(modules
                .get("host", ModuleType::RequestHandler)?
                .build(engine, &ModuleConfig::RequestHandler(self.clone()))?),
            HandlerBuilder::FileServer { .. } => Ok(modules
                .get("file_server", ModuleType::RequestHandler)?
                .build(engine, &ModuleConfig::RequestHandler(self.clone()))?),
        }
    }
}
