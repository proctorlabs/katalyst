use super::*;
use crate::app::KatalystEngine;
use crate::authentication::AuthenticatorDirectory;
use crate::error::ConfigurationFailure;
use serde::{Deserialize, Serialize};
use std::string::String;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AuthenticatorBuilder {
    backend: String,
    pub url: Option<String>,
}

impl Builder<Authenticator> for AuthenticatorBuilder {
    fn build(&self, engine: Arc<KatalystEngine>) -> Result<Authenticator, ConfigurationFailure> {
        let authenticators = engine.locate::<AuthenticatorDirectory>()?;
        Ok(Authenticator {
            authenticator: authenticators
                .get(&self.backend.as_str())
                .ok_or_else(|| {
                    ConfigurationFailure::ExpressionItemNotFound("authenticator".to_string())
                })?
                .build(self),
        })
    }
}
