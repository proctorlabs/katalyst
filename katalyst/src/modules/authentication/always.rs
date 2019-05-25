use crate::app::Katalyst;
use crate::context::*;
use crate::modules::*;
use futures::future::*;

#[derive(Default, Debug)]
pub struct AlwaysAuthenticatorBuilder;

impl ModuleProvider for AlwaysAuthenticatorBuilder {
    fn name(&self) -> &'static str {
        "always"
    }

    fn build(&self, _: ModuleType, _: Arc<Katalyst>, _: &unstructured::Document) -> Result<Module> {
        Ok(Module::Authenticator(Arc::new(AlwaysAuthenticator)))
    }
}

#[derive(Default, Debug)]
pub struct AlwaysAuthenticator;

impl RequestHook for AlwaysAuthenticator {
    fn run(&self, mut ctx: Context) -> ModuleResult {
        let mut result = KatalystAuthenticationInfo::default();
        result.add_claim("KatalystAuthenticator".to_string(), "always".to_string());
        ctx = match ctx.set_authenticated(result) {
            Ok(c) => c,
            Err(e) => return Box::new(err(e)),
        };
        Box::new(ok::<Context, ModuleError>(ctx))
    }
}
