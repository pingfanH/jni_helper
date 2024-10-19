extern crate jni_helper;

use jni_helper::java;
//include!("examples/java_jni.rs");
//include the java_jni.rs in your lib
fn main() {
    let  java_path = "examples/RustNative.java"; // Kotlin 文件所在的路径
    java(java_path,"examples/java_jni.rs");
}