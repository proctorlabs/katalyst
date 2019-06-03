use crate::{app::Katalyst, context::*, modules::*, prelude::*};

#[derive(Default, Debug)]
pub struct AlwaysAuthenticator;

impl ModuleProvider for AlwaysAuthenticator {
    fn name(&self) -> &'static str {
        "always"
    }

    fn build(&self, _: ModuleType, _: Arc<Katalyst>, _: &unstructured::Document) -> Result<Module> {
        Ok(AlwaysAuthenticator.into_module())
    }
}

impl AuthenticatorModule for AlwaysAuthenticator {
    fn authenticate(&self, guard: ContextGuard) -> AsyncResult<()> {
        let mut result = KatalystAuthenticationInfo::default();
        result.add_claim("KatalystAuthenticator".to_string(), "always".to_string());
        guard.set_authenticated(result).fut()
    }
}
