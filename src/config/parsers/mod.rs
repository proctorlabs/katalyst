use crate::config::builder::GatewayBuilder;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod yaml;

pub fn parse_yaml_file(yaml_file: &str) -> GatewayBuilder {
    let contents = load_file(yaml_file);
    yaml::process_yaml(contents)
}

fn load_file<'a>(file_path: &'a str) -> String {
    let path = Path::new(file_path);

    println!(
        "Loading file from: {}",
        path.canonicalize().unwrap().display()
    );

    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}
