use crate::app::Katalyst;
use crate::context::*;
use crate::modules::*;
use futures::future::*;

#[derive(Default, Debug)]
pub struct AlwaysAuthenticatorBuilder {}

impl Module for AlwaysAuthenticatorBuilder {
    fn name(&self) -> &'static str {
        "always"
    }

    fn supported_hooks(&self) -> Vec<ModuleType> {
        vec![ModuleType::Authenticator]
    }

    fn build_hook(
        &self,
        _: ModuleType,
        _: Arc<Katalyst>,
        _: &unstructured::Document,
    ) -> Result<Arc<ModuleDispatch>, ConfigurationFailure> {
        Ok(Arc::new(AlwaysAuthenticator {}))
    }
}

#[derive(Default, Debug)]
pub struct AlwaysAuthenticator {}

impl ModuleDispatch for AlwaysAuthenticator {
    fn dispatch(&self, mut ctx: Context) -> ModuleResult {
        let mut result = KatalystAuthenticationInfo::default();
        result.add_claim("KatalystAuthenticator".to_string(), "always".to_string());
        ctx.detail.authentication = Some(result);
        Box::new(ok::<Context, ModuleError>(ctx))
    }
}
