mod always;
mod http;
mod never;

use crate::config::builder::AuthenticatorBuilder;
use crate::error::KatalystError;
use crate::pipeline::AsyncPipelineResult;
use crate::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

pub type AuthenticatorDirectory = HashMap<&'static str, Arc<KatalystAuthenticatorBuilder>>;

#[derive(Debug, Default)]
pub struct KatalystAuthenticationInfo {
    claims: HashMap<String, Vec<String>>,
}

impl KatalystAuthenticationInfo {
    pub fn add_claim(&mut self, claim_type: String, claim_value: String) {
        if let Some(claims) = self.claims.get_mut(&claim_type) {
            claims.push(claim_value);
        } else {
            self.claims.insert(claim_type, vec![claim_value]);
        }
    }

    pub fn get_claim(&self, claim_type: String) -> String {
        match self.claims.get(&claim_type) {
            Some(c) => c[0].to_string(),
            None => String::default(),
        }
    }
}

pub trait KatalystAuthenticatorBuilder: Send + Sync + Debug {
    fn name(&self) -> &'static str;

    fn build(&self, config: &AuthenticatorBuilder) -> Arc<KatalystAuthenticator>;
}

pub trait KatalystAuthenticator: Send + Sync + Debug {
    fn authenticate(&self, ctx: Context) -> AsyncPipelineResult;
}

pub(crate) fn all() -> AuthenticatorDirectory {
    let mut result: AuthenticatorDirectory = HashMap::new();
    let mut authenticators: Vec<Arc<KatalystAuthenticatorBuilder>> = vec![
        always::AlwaysAuthenticatorBuilder::arc(),
        never::NeverAuthenticatorBuilder::arc(),
        http::HttpAuthenticatorBuilder::arc(),
    ];
    while let Some(authenticator) = authenticators.pop() {
        result.insert(authenticator.name(), authenticator);
    }
    result
}
