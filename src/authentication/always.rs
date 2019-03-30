use crate::authentication::*;
use futures::future::ok;

#[derive(Default, Debug)]
pub struct AlwaysAuthenticatorBuilder {}

impl KatalystAuthenticatorBuilder for AlwaysAuthenticatorBuilder {
    fn name(&self) -> &'static str {
        "always"
    }

    fn build(&self, _: &AuthenticatorBuilder) -> Arc<KatalystAuthenticator> {
        Arc::new(AlwaysAuthenticator {})
    }
}

#[derive(Default, Debug)]
pub struct AlwaysAuthenticator {}

impl KatalystAuthenticator for AlwaysAuthenticator {
    fn authenticate(&self, mut state: PipelineState) -> AsyncPipelineResult {
        let mut result = KatalystAuthenticationInfo::default();
        result.add_claim("KatalystAuthenticator".to_string(), "always".to_string());
        state.context.authentication = Some(result);
        Box::new(ok::<PipelineState, KatalystError>(state))
    }
}
