use katalyst;
use log::Level;

fn main() {
    simple_logger::init_with_level(Level::Error).unwrap();
    katalyst::start_katalyst("/katalyst.yml");
}
