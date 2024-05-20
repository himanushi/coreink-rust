extern crate bindgen;

use std::path::PathBuf;

fn main() {
    embuild::espidf::sysenv::output();

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-Iinclude")
        .clang_arg("-Iesp-idf/components/esp_rom/include")
        .clang_arg("-Iesp-idf/components/esp_driver_spi/include")
        .clang_arg("-Iesp-idf/components/esp_hw_support/include")
        .clang_arg("-Iesp-idf/components/esp_common/include")
        .clang_arg("-Iesp-idf/components/esp_system/include")
        .clang_arg("-Iesp-idf/components/hal/include")
        .clang_arg("-Iesp-idf/components/soc/esp32/include")
        .clang_arg("-Iesp-idf/components/freertos/FreeRTOS-Kernel/include")
        .clang_arg("-Iesp-idf/components/freertos/config/include")
        .clang_arg("-Iesp-idf/components/freertos/esp_additions/include")
        .clang_arg("-Iesp-idf/components/heap/include")
        .clang_arg("-Iesp-idf/components/esp_pm/include")
        .clang_arg("-Iesp-idf/components/soc/include")
        .clang_arg("-Iesp-idf/components/freertos/config/include/freertos")
        .clang_arg("-Iesp-idf/components/freertos/config/linux/include")
        .clang_arg("-I/usr/lib/gcc/aarch64-linux-gnu/12/include")
        .clang_arg("-I/usr/include/c++/12")
        .clang_arg("-I/usr/include/aarch64-linux-gnu/c++/12")
        .clang_arg("-I/usr/include/c++/12/backward")
        .clang_arg("-I/usr/local/include")
        .clang_arg("-I/usr/include/aarch64-linux-gnu")
        .clang_arg("-I/usr/include")
        .clang_arg("-D_GNU_SOURCE")
        .clang_arg("-xc++")
        .clang_arg("-std=c++14")
        .clang_arg("-lpthread")
        .clang_arg("-lc")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
