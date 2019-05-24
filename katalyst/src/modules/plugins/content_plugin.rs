use crate::app::Katalyst;
use crate::context::*;
use crate::modules::*;

#[derive(Debug)]
pub struct ContentPlugin;

impl Module for ContentPlugin {
    fn name(&self) -> &'static str {
        "parse-content"
    }

    fn supported_hooks(&self) -> Vec<ModuleType> {
        vec![ModuleType::Plugin]
    }

    fn build_hook(
        &self,
        _: ModuleType,
        _: Arc<Katalyst>,
        _: &unstructured::Document,
    ) -> Result<Arc<ModuleDispatch>> {
        Ok(Arc::new(ContentPlugin {}))
    }
}

impl ModuleDispatch for ContentPlugin {
    fn dispatch(&self, ctx: Context) -> ModuleResult {
        ctx.parse()
    }
}
