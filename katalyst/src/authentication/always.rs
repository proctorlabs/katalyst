use crate::authentication::*;
use crate::prelude::*;
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
    fn authenticate(&self, mut ctx: Context) -> AsyncPipelineResult {
        let mut result = KatalystAuthenticationInfo::default();
        result.add_claim("KatalystAuthenticator".to_string(), "always".to_string());
        ctx.detail.authentication = Some(result);
        Box::new(ok::<Context, KatalystError>(ctx))
    }
}
