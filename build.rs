fn main() {
    // ESP-IDF環境変数の出力
    embuild::espidf::sysenv::output();

    // ESP-IDFのパスを取得
    let idf_path = std::env::var("IDF_PATH").expect("IDF_PATH environment variable is not set");

    // 標準ライブラリのインクルードパスを設定
    let cpp_include_path1 = "/opt/xtensa-esp32-elf/xtensa-esp32-elf/include/c++/8.4.0";
    let cpp_include_path2 = "/opt/xtensa-esp32-elf/xtensa-esp32-elf/include/c++/8.4.0/xtensa-esp32-elf";
    let cpp_include_path3 = "/opt/xtensa-esp32-elf/lib/gcc/xtensa-esp32-elf/8.4.0/include";

    // xtensa-esp32-elf-g++コンパイラを使用してC++ライブラリのビルド
    cc::Build::new()
        .cpp(true)
        .compiler("xtensa-esp32-elf-g++")
        .flag("-std=c++11")
        .flag("-fno-exceptions") 
        .flag("-fno-rtti")
        .include("c_libs/M5GFX/src")
        .include("c_libs/M5Unified/src")
        .include(cpp_include_path1)
        .include(cpp_include_path2)
        .include(cpp_include_path3)
        .file("c_libs/M5GFX/src/M5GFX.cpp")
        .file("c_libs/M5Unified/src/M5Unified.cpp")
        .compile("m5");

    // バインディングの生成
    println!("cargo:rerun-if-changed=wrapper.h");

    bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-xc++") // C++としてバインドを生成するように指定
        .clang_arg(format!("-I{}/components/esp_common/include", idf_path)) // ESP-IDFのヘッダーを含める
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
