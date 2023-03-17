use std::{path::{PathBuf, Path}, env};

fn main( ) {
    println!("cargo:rerun-if-changed=templates");
    let output_path = get_output_path();
    let input_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("templates");
    let output_path = Path::new(&output_path).join("templates");
    let res = std::fs::copy(input_path, output_path);
    println!("cargo:warning={:#?}",res);
}

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string).join("target").join(build_type);
    return PathBuf::from(path);
}