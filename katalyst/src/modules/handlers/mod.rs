mod files;
mod host;
use crate::modules::*;

pub use files::FileServerModule;
pub use host::HostModule;

#[derive(Default, Clone, Debug)]
pub struct HandlerModule;

impl ModuleProviderDefinition for HandlerModule {
    const MODULE_TYPE: ModuleType = ModuleType::RequestHandler;
    type ModuleImplType = Arc<ModuleDispatch>;
}
