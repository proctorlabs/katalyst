use super::*;
use crate::app::KatalystEngine;
use crate::error::ConfigurationFailure;
use crate::modules::*;
use serde::{Deserialize, Serialize};
use std::string::String;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum AuthenticatorBuilder {
    Always,
    Never,
    Http { url: Option<String> },
}

impl Builder<Arc<ModuleDispatch>> for AuthenticatorBuilder {
    fn build(
        &self,
        engine: Arc<KatalystEngine>,
    ) -> Result<Arc<ModuleDispatch>, ConfigurationFailure> {
        let modules: Arc<Modules> = engine.locate()?;
        match self {
            AuthenticatorBuilder::Always { .. } => Ok(modules
                .get("always", ModuleType::Authenticator)?
                .build(engine, &ModuleConfig::Authenticator(self.clone()))?),
            AuthenticatorBuilder::Never { .. } => Ok(modules
                .get("never", ModuleType::Authenticator)?
                .build(engine, &ModuleConfig::Authenticator(self.clone()))?),
            AuthenticatorBuilder::Http { .. } => Ok(modules
                .get("http", ModuleType::Authenticator)?
                .build(engine, &ModuleConfig::Authenticator(self.clone()))?),
        }
    }
}
