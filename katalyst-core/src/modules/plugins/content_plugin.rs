use crate::{modules::*, Katalyst};

#[derive(Debug)]
pub struct ContentPlugin;

impl ModuleProvider for ContentPlugin {
    fn name(&self) -> &'static str {
        "parse-content"
    }

    fn build(&self, _: ModuleType, _: Katalyst, _: &unstructured::Document) -> Result<Module> {
        Ok(ContentPlugin.into_module())
    }
}

impl PluginModule for ContentPlugin {
    fn run(&self, guard: RequestContext) -> ModuleResult {
        guard.parse()
    }
}
