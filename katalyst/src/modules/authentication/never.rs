use crate::app::Katalyst;
use crate::context::*;
use crate::modules::*;
use futures::future::err;

#[derive(Default, Debug)]
pub struct NeverAuthenticatorBuilder;

impl ModuleProvider for NeverAuthenticatorBuilder {
    fn name(&self) -> &'static str {
        "never"
    }

    fn build(&self, _: ModuleType, _: Arc<Katalyst>, _: &unstructured::Document) -> Result<Module> {
        Ok(Module::Authenticator(Arc::new(NeverAuthenticator)))
    }
}

#[derive(Default, Debug)]
pub struct NeverAuthenticator;

impl RequestHook for NeverAuthenticator {
    fn run(&self, ctx: Context) -> ModuleResult {
        Box::new(err(ctx.fail(GatewayError::Unauthorized)))
    }
}
