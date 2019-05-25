mod content_plugin;
use crate::modules::*;

pub use content_plugin::ContentPlugin;

#[derive(Default, Clone, Debug)]
pub struct PluginModule;

impl ModuleProviderDefinition for PluginModule {
    const MODULE_TYPE: ModuleType = ModuleType::Plugin;
    type ModuleImplType = Arc<ModuleDispatch>;
}
