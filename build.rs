extern crate bindgen;

use std::path::PathBuf;

fn main() {
    embuild::espidf::sysenv::output();

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-Iesp-idf/components/esp_rom/include")
        .clang_arg("-Iesp-idf/components/esp_driver_spi/include")
        .clang_arg("-Iesp-idf/components/esp_hw_support/include")
        .clang_arg("-Iesp-idf/components/esp_common/include")
        .clang_arg("-Iesp-idf/components/esp_system/include")
        .clang_arg("-Iesp-idf/components/esp_system/include")
        .clang_arg("-Iesp-idf/components/hal/include")
        .clang_arg("-Iesp-idf/components/soc/esp32/include")
        .clang_arg("-Iclibs/M5GFX/src")
        .clang_arg("-Iclibs/M5Unified/src")
        .clang_arg("-I.")
        .clang_arg("-I/usr/lib/gcc/aarch64-linux-gnu/12/include")
        .clang_arg("-I/usr/local/include")
        .clang_arg("-I/usr/include/aarch64-linux-gnu")
        .clang_arg("-I/usr/include")
        .clang_arg("-xc++")
        .clang_arg("-std=c++14")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
