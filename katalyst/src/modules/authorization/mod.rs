use crate::modules::*;

#[derive(Default, Clone, Debug)]
pub struct AuthorizerModule {}
impl PhantomModuleData for AuthorizerModule {
    const MODULE_TYPE: ModuleType = ModuleType::Authorizer;
    type ModuleImpl = Arc<ModuleDispatch>;
}
