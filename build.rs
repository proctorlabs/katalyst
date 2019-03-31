use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Read, Write};
use cargo_readme;

//This isn't necessary for the library itself, but allows the examples to be easily run.
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let config_file = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut dest_path = Path::new(&out_dir).to_path_buf();
    let mut src_path = Path::new(&config_file).to_path_buf();

    //Make source the examples config.yml
    src_path.push("examples");
    src_path.push("config.yml");

    //Make destination examples directory, which is relative three directories up from katalyst target
    dest_path.pop();
    dest_path.pop();
    dest_path.pop();
    dest_path.push("examples");
    dest_path.push("config.yml");

    //Finally, copy the file over
    println!("Copying {} to {}", src_path.to_str().unwrap(), dest_path.to_str().unwrap());
    fs::copy(src_path, dest_path).unwrap();
    build_readme();
}


fn build_readme() {
    let mut f = fs::File::open("src/lib.rs").unwrap();
    let mut t = fs::File::open("README.tpl").unwrap();
    let content = cargo_readme::generate_readme(
        &PathBuf::from("./"),
        &mut f,
        Some(&mut t),
        false,
        false,
        false,
        true,
    ).unwrap();

    let mut orig = String::new();
    f.read_to_string(&mut orig).unwrap();
    if orig != content.as_str().to_string() {
        let mut f = fs::File::create("README.md").unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }
}