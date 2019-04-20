use crate::app::KatalystEngine;
use crate::context::*;
use crate::modules::*;
use crate::prelude::*;
use futures::future::err;

#[derive(Default, Debug)]
pub struct NeverAuthenticatorBuilder {}

impl Module for NeverAuthenticatorBuilder {
    fn name(&self) -> &'static str {
        "never"
    }

    fn module_type(&self) -> ModuleType {
        ModuleType::Authenticator
    }

    fn build(
        &self,
        _: Arc<KatalystEngine>,
        _: &ModuleConfigLoader,
    ) -> Result<Arc<ModuleDispatch>, ConfigurationFailure> {
        Ok(Arc::new(NeverAuthenticator {}))
    }
}

#[derive(Default, Debug)]
pub struct NeverAuthenticator {}

impl ModuleDispatch for NeverAuthenticator {
    fn dispatch(&self, _: Context) -> ModuleResult {
        Box::new(err::<Context, RequestFailure>(RequestFailure::Unauthorized))
    }
}
