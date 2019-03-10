use crate::authentication::*;

#[derive(Default, Debug)]
pub struct AlwaysAuthenticator {}

impl KatalystAuthenticator for AlwaysAuthenticator {
    fn name(&self) -> &'static str {
        "always"
    }

    fn authenticate(&self, _: &PipelineState) -> AuthenticationResult {
        Ok(KatalystAuthenticationInfo::default())
    }
}

#[derive(Default, Debug)]
pub struct NeverAuthenticator {}

impl KatalystAuthenticator for NeverAuthenticator {
    fn name(&self) -> &'static str {
        "never"
    }

    fn authenticate(&self, _: &PipelineState) -> AuthenticationResult {
        Err(KatalystError::Unauthorized)
    }
}
