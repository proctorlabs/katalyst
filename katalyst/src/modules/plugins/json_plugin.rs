use crate::app::KatalystEngine;
use crate::modules::*;
use crate::prelude::*;

#[derive(Debug)]
pub struct JsonPlugin {}

impl Module for JsonPlugin {
    fn name(&self) -> &'static str {
        "json"
    }

    fn module_type(&self) -> ModuleType {
        ModuleType::Plugin
    }

    fn build(
        &self,
        _: Arc<KatalystEngine>,
        _: &ModuleConfigLoader,
    ) -> Result<Arc<ModuleDispatch>, ConfigurationFailure> {
        Ok(Arc::new(JsonPlugin {}))
    }
}

impl ModuleDispatch for JsonPlugin {
    fn dispatch(&self, ctx: Context) -> ModuleResult {
        ok!(ctx)
    }
}
