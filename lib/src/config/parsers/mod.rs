use crate::config::builder::GatewayBuilder;
use serde_yaml;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn parse_file(file_path: &str) -> GatewayBuilder {
    let path = Path::new(file_path);
    let contents = load_file(path);

    match path.extension().and_then(OsStr::to_str) {
        Some("yml") | Some("yaml") => serde_yaml::from_str(&contents).unwrap(),
        Some("json") | Some("js") => serde_json::from_str(&contents).unwrap(),
        _ => panic!("Unrecognized config file format!"),
    }
}

fn load_file<'a>(path: &Path) -> String {
    info!(
        "Loading file from: {}",
        path.canonicalize().unwrap().display()
    );

    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
