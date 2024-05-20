extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    embuild::espidf::sysenv::output();

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/path/to/clibs/M5GFX/src")
        .clang_arg("-I/path/to/clibs/M5Unified/src")
        .clang_arg("-I/path/to/your_project_name")
        .clang_arg("-xc++")
        .clang_arg("-std=c++14")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
