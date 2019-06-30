use crate::{app::KatalystCore, modules::*};
use futures::future::err;

#[derive(Default, Debug)]
pub struct NeverAuthenticator;

impl ModuleProvider for NeverAuthenticator {
    fn name(&self) -> &'static str {
        "never"
    }

    fn build(
        &self,
        _: ModuleType,
        _: Arc<KatalystCore>,
        _: &unstructured::Document,
    ) -> Result<Module> {
        Ok(NeverAuthenticator.into_module())
    }
}

impl AuthenticatorModule for NeverAuthenticator {
    fn authenticate(&self, _: RequestContext) -> ModuleResult {
        fail!(:FORBIDDEN, "This module always rejects requests")
    }
}
