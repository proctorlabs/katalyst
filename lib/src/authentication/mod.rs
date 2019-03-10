mod testing;

use crate::common::KatalystCommonUtilities;
use crate::error::KatalystError;
use crate::pipeline::PipelineState;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

pub type AuthenticationResult = Result<(KatalystAuthenticationInfo), KatalystError>;
pub type AuthenticatorDirectory = HashMap<&'static str, Arc<KatalystAuthenticator>>;

#[derive(Debug, Default)]
pub struct KatalystAuthenticationInfo {}

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
