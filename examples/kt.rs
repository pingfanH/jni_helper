extern crate jni_helper;

use jni_helper::kt;
//include!("examples/kt_jni.rs");
//include the kt_jni.rs in your lib
fn main() {
    let kt_path = "RustNative.kt"; // Kotlin File Path
    kt(kt_path,"your/kotlinc/path/kotlinc.bat","src/kt_jni.rs");
}