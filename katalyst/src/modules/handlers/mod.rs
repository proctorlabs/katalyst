mod files;
mod host;
use crate::modules::*;

pub use files::FileServerModule;
pub use host::HostModule;

#[derive(Default, Clone, Debug)]
pub struct HandlerModule {}

impl ModuleProvider for HandlerModule {
    const MODULE_TYPE: ModuleType = ModuleType::RequestHandler;

    type ModuleImplType = Arc<ModuleDispatch>;

    fn build(
        module: Arc<Module>,
        instance: Arc<Katalyst>,
        doc: &unstructured::Document,
    ) -> Result<Self::ModuleImplType> {
        module.build_hook(Self::MODULE_TYPE, instance, doc)
    }
}
