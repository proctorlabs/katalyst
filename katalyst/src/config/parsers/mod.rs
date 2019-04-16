use crate::config::builder::KatalystBuilder;
use crate::prelude::*;
use serde_yaml;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn parse_file(file_path: &str) -> Result<KatalystBuilder, ConfigurationFailure> {
    let path = Path::new(file_path);
    let contents = load_file(path)?;

    match path.extension().and_then(OsStr::to_str) {
        Some("yml") | Some("yaml") => Ok(serde_yaml::from_str(&contents)?),
        Some("json") | Some("js") => Ok(serde_json::from_str(&contents)?),
        _ => Ok(serde_yaml::from_str(&contents)?),
    }
}

fn load_file(path: &Path) -> Result<String, ConfigurationFailure> {
    info!("Loading file from: {}", path.canonicalize()?.display());

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
