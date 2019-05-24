mod content_plugin;
use crate::modules::*;

pub use content_plugin::ContentPlugin;

#[derive(Default, Clone, Debug)]
pub struct PluginModule;

impl ModuleProvider for PluginModule {
    const MODULE_TYPE: ModuleType = ModuleType::Plugin;

    type ModuleImplType = Arc<ModuleDispatch>;

    fn build(
        module: Arc<Module>,
        instance: Arc<Katalyst>,
        doc: &unstructured::Document,
    ) -> Result<Self::ModuleImplType> {
        module.build_hook(Self::MODULE_TYPE, instance, doc)
    }
}
