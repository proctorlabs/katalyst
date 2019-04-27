use crate::config::builder::KatalystBuilder;
use crate::prelude::*;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn parse_file(file_path: &str) -> Result<KatalystBuilder, ConfigurationFailure> {
    let path = Path::new(file_path);
    let contents = load_file(path)?;

    Ok(Parser::from_str(
        &contents,
        Format::ext(path.extension().and_then(OsStr::to_str)),
    )?)
}

fn load_file(path: &Path) -> Result<String, ConfigurationFailure> {
    info!("Loading file from: {}", path.canonicalize()?.display());

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
