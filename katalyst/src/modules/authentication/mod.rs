mod always;
mod http;
mod never;
mod whitelist;
use crate::modules::*;

pub use self::http::HttpAuthenticatorBuilder;
pub use always::AlwaysAuthenticatorBuilder;
pub use never::NeverAuthenticatorBuilder;
pub use whitelist::WhitelistBuilder;

#[derive(Default, Clone, Debug)]
pub struct AuthenticatorModule {}

impl ModuleProvider for AuthenticatorModule {
    const MODULE_TYPE: ModuleType = ModuleType::Authenticator;

    type ModuleImplType = Arc<ModuleDispatch>;

    fn build(
        module: Arc<Module>,
        instance: Arc<Katalyst>,
        doc: &unstructured::Document,
    ) -> Result<Self::ModuleImplType, ConfigurationFailure> {
        module.build_hook(Self::MODULE_TYPE, instance, doc)
    }
}
