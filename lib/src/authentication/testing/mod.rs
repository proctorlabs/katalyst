use crate::authentication::*;

#[derive(Default, Debug)]
pub struct AlwaysAuthenticator {}

impl KatalystAuthenticator for AlwaysAuthenticator {
    fn name(&self) -> &'static str {
        "always"
    }

    fn authenticate(&self, _: &PipelineState) -> AuthenticationResult {
        let mut result = KatalystAuthenticationInfo::default();
        result.add_claim("KatalystAuthenticator".to_string(), self.name().to_string());
        Ok(result)
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
