include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    unsafe {
        // M5Unifiedの関数を呼び出す例
        M5.begin();
    }
}
