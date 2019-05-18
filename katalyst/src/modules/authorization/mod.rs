use crate::modules::*;

#[derive(Default, Clone, Debug)]
pub struct AuthorizerModule {}

impl ModuleProvider for AuthorizerModule {
    const MODULE_TYPE: ModuleType = ModuleType::Authorizer;

    type ModuleImplType = Arc<ModuleDispatch>;

    fn build(
        module: Arc<Module>,
        instance: Arc<Katalyst>,
        doc: &unstructured::Document,
    ) -> Result<Self::ModuleImplType> {
        module.build_hook(Self::MODULE_TYPE, instance, doc)
    }
}
