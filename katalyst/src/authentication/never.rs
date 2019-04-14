use crate::authentication::*;
use crate::prelude::*;
use futures::future::err;

#[derive(Default, Debug)]
pub struct NeverAuthenticatorBuilder {}

impl KatalystAuthenticatorBuilder for NeverAuthenticatorBuilder {
    fn name(&self) -> &'static str {
        "never"
    }

    fn build(&self, _: &AuthenticatorBuilder) -> Arc<KatalystAuthenticator> {
        Arc::new(NeverAuthenticator {})
    }
}

#[derive(Default, Debug)]
pub struct NeverAuthenticator {}

impl KatalystAuthenticator for NeverAuthenticator {
    fn authenticate(&self, _: Context) -> AsyncPipelineResult {
        Box::new(err::<Context, RequestFailure>(RequestFailure::Unauthorized))
    }
}
