use crate::modules::*;

#[derive(Default, Clone, Debug)]
pub struct AuthorizerModule;

impl ModuleProviderDefinition for AuthorizerModule {
    const MODULE_TYPE: ModuleType = ModuleType::Authorizer;
    type ModuleImplType = Arc<dyn RequestHook>;
}
