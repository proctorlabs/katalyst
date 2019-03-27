#[macro_use]
extern crate log;

use katalyst::Katalyst;
use log::Level;
use std::env;
use std::io;
use std::path::PathBuf;

fn config_path() -> io::Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    dir.push("config.yml");
    Ok(dir)
}

fn main() {
    simple_logger::init_with_level(Level::Debug).unwrap();
    let path_buf = config_path().expect("Couldn't create path");
    let path = path_buf.to_string_lossy();
    info!("Loading file from {}", &path);
    Katalyst::start(&path).unwrap();
}
