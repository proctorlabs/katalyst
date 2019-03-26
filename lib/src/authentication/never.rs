use crate::authentication::*;
use futures::future::err;

#[derive(Default, Debug)]
pub struct NeverAuthenticatorBuilder {}

impl KatalystAuthenticatorBuilder for NeverAuthenticatorBuilder {
    fn name(&self) -> &'static str {
        "never"
    }

    fn build(&self) -> Arc<KatalystAuthenticator> {
        Arc::new(NeverAuthenticator {})
    }
}

#[derive(Default, Debug)]
pub struct NeverAuthenticator {}

impl KatalystAuthenticator for NeverAuthenticator {
    fn authenticate(&self, _: PipelineState) -> AsyncPipelineResult {
        Box::new(err::<PipelineState, KatalystError>(
            KatalystError::Unauthorized,
        ))
    }
}
