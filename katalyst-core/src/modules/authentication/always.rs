use crate::{app::KatalystCore, modules::*, prelude::*};

#[derive(Default, Debug)]
pub struct AlwaysAuthenticator;

impl ModuleProvider for AlwaysAuthenticator {
    fn name(&self) -> &'static str {
        "always"
    }

    fn build(
        &self,
        _: ModuleType,
        _: Arc<KatalystCore>,
        _: &unstructured::Document,
    ) -> Result<Module> {
        Ok(AlwaysAuthenticator.into_module())
    }
}

impl AuthenticatorModule for AlwaysAuthenticator {
    fn authenticate(&self, guard: RequestContext) -> AsyncResult<()> {
        let mut result = Authentication::Authenticated { claims: HashMap::default() };
        result.add_claim("KatalystCoreAuthenticator".to_string(), "always".to_string());
        guard.set_authentication(result).fut()
    }
}
