use crate::app::Katalyst;
use crate::context::*;
use crate::modules::*;
use futures::future::err;

#[derive(Default, Debug)]
pub struct NeverAuthenticatorBuilder;

impl Module for NeverAuthenticatorBuilder {
    fn name(&self) -> &'static str {
        "never"
    }

    fn supported_hooks(&self) -> Vec<ModuleType> {
        vec![ModuleType::Authenticator]
    }

    fn build_hook(
        &self,
        _: ModuleType,
        _: Arc<Katalyst>,
        _: &unstructured::Document,
    ) -> Result<Arc<ModuleDispatch>> {
        Ok(Arc::new(NeverAuthenticator {}))
    }
}

#[derive(Default, Debug)]
pub struct NeverAuthenticator;

impl ModuleDispatch for NeverAuthenticator {
    fn dispatch(&self, ctx: Context) -> ModuleResult {
        Box::new(err(ctx.fail(GatewayError::Unauthorized)))
    }
}
