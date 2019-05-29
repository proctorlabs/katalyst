use crate::app::Katalyst;
use crate::context::*;
use crate::modules::*;
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
    fn authenticate(&self, ctx: Context) -> ModuleResult {
        Box::new(err(ctx.fail(GatewayError::Unauthorized)))
    }
}
