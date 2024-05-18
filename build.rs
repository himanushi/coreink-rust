fn main() {
    embuild::espidf::sysenv::output();

    cc::Build::new()
        .file("c_libs/M5Unified/src/M5Unified.cpp")
        .compile("M5Unified");
}
