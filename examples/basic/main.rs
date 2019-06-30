use katalyst::{prelude::*, *};
use log::Level;

fn main() -> Result<()> {
    let config = include_str!("config.yml");
    simple_logger::init_with_level(Level::Debug).unwrap();
    let katalyst = Katalyst::new()?;
    katalyst.load_yaml(config)?;
    katalyst.run()
}
