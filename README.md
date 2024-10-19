# Jni Function Descriptor Generate
## Required
 - jdk
 - kotlinc(kotlin)

## Usage
 -  create build.rs in you project root
    ```rust
    use jni_helper;
    fn main() {
    jni_helper::kt("your/path/RustNative.kt","your/bin/kotlinc.bat","src/kt_jni.rs");//kt
    jni_helper::java("your/path/RustNative.java","your/bin/kotlinc.bat","src/java_jni.rs");//java
    jni_helper::custom("your/path/RustNative.kt",vec!["your/bin/javac","-d","classes/java","-encoding","UTF-8"],"src/java_jni.rs");//custom
    }
    ```
 - **include!()** jni.rs in your lib.rs
 - PS:If there are Chinese characters in the Java file, you need to include "-encoding", "UTF-8" in custom javac
# [Demo](https://github.com/pingfanH/rust_android_jni_demo)