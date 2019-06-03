use crate::{app::Katalyst, context::*, modules::*};
use futures::future::err;

#[derive(Default, Debug)]
pub struct NeverAuthenticator;

impl ModuleProvider for NeverAuthenticator {
    fn name(&self) -> &'static str {
        "never"
    }

    fn build(&self, _: ModuleType, _: Arc<Katalyst>, _: &unstructured::Document) -> Result<Module> {
        Ok(NeverAuthenticator.into_module())
    }
}

impl AuthenticatorModule for NeverAuthenticator {
    fn authenticate(&self, _: ContextGuard) -> ModuleResult {
        Box::new(err(GatewayError::Unauthorized))
    }
}
