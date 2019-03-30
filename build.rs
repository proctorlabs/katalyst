use std::env;
use std::fs;
use std::path::Path;

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
}