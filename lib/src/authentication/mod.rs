mod testing;

use crate::common::KatalystCommonUtilities;
use crate::error::KatalystError;
use crate::pipeline::PipelineState;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

pub type AuthenticationResult = Result<KatalystAuthenticationInfo, KatalystError>;
pub type AuthenticatorDirectory = HashMap<&'static str, Arc<KatalystAuthenticator>>;

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

pub trait KatalystAuthenticator: Send + Sync + Debug {
    fn name(&self) -> &'static str;

    fn authenticate(&self, state: &PipelineState) -> AuthenticationResult;
}

pub fn all() -> AuthenticatorDirectory {
    let mut result: AuthenticatorDirectory = HashMap::new();
    let mut authenticators: Vec<Arc<KatalystAuthenticator>> = vec![
        testing::AlwaysAuthenticator::arc(),
        testing::NeverAuthenticator::arc(),
    ];
    while let Some(authenticator) = authenticators.pop() {
        result.insert(authenticator.name(), authenticator);
    }
    result
}