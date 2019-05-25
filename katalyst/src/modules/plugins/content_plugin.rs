use crate::app::Katalyst;
use crate::context::*;
use crate::modules::*;

#[derive(Debug)]
pub struct ContentPlugin;

impl ModuleProvider for ContentPlugin {
    fn name(&self) -> &'static str {
        "parse-content"
    }

    fn build(&self, _: ModuleType, _: Arc<Katalyst>, _: &unstructured::Document) -> Result<Module> {
        Ok(Module::Plugin(Arc::new(ContentPlugin)))
    }
}

impl RequestHook for ContentPlugin {
    fn run(&self, ctx: Context) -> ModuleResult {
        ctx.parse()
    }
}
