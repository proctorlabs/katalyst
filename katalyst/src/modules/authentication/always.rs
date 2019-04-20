use crate::app::KatalystEngine;
use crate::context::*;
use crate::modules::*;
use crate::prelude::*;
use futures::future::*;

#[derive(Default, Debug)]
pub struct AlwaysAuthenticatorBuilder {}

impl Module for AlwaysAuthenticatorBuilder {
    fn name(&self) -> &'static str {
        "always"
    }

    fn module_type(&self) -> ModuleType {
        ModuleType::Authenticator
    }

    fn build(
        &self,
        _: Arc<KatalystEngine>,
        _: &ModuleConfigLoader,
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
        Box::new(ok::<Context, RequestFailure>(ctx))
    }
}
