mod content_plugin;
use crate::modules::*;

pub use content_plugin::ContentPlugin;

#[derive(Default, Clone, Debug)]
pub struct PluginModule {}
impl PhantomModuleData for PluginModule {
    const MODULE_TYPE: ModuleType = ModuleType::Plugin;
}
