use katalyst::{error::*, prelude::*, Document, Katalyst};
use log::Level;

#[macro_use]
extern crate katalyst;

katalyst_link! {
    modules: {
        HelloWorldProvider
    }
}

#[derive(Debug, Default)]
pub struct HelloWorldProvider;

impl ModuleProvider for HelloWorldProvider {
    fn name(&self) -> &'static str {
        "hello_world"
    }

    fn build(&self, _: ModuleType, _: &Document) -> Result<Module> {
        Ok(HelloWorld {}.into_module())
    }
}

#[derive(Debug)]
pub struct HelloWorld {}

impl RequestHandlerModule for HelloWorld {
    fn dispatch(&self, _: RequestContext) -> ModuleResult {
        unimplemented!();
    }
}

fn main() -> Result<()> {
    let config = include_str!("config.yml");
    simple_logger::init_with_level(Level::Debug).unwrap();
    katalyst_load!(modules: { crate });
    let katalyst = Katalyst::new()?;
    katalyst.load_yaml(config)?;
    katalyst.run()
}
