pub mod fun;
mod utils;

use fun::Fun;
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use regex::Regex;

extern crate regex;
use utils::*;
///
/// # Java
/// - file_path: java file path
/// - rs: generated rs file path
pub fn java(file_path:&str,rs:&str){
    main(file_path,vec!["javac","-encoding","UTF-8","-d","target/classes"],rs);
}
///
/// # Kotlin
/// - file_path: kt file path
/// - compiler: compiler path
/// - rs: generated rs file path
pub fn kt(file_path:&str,compiler:&str,rs:&str){
    // #[cfg(target_os = "windows")]
    // let kotlinc_path = format!("{}\\kotlinc.bat", compiler);
    // #[cfg(not(target_os = "windows"))]
    // let kotlinc_path = format!("{}\\kotlinc", kotlinc_path);
    main(file_path,vec![compiler,"-d","target/classes"],rs);
}
///
/// # Custom
/// - file_path: kt/java file path
/// - compiler: compiler args
/// - rs: generated rs file path
pub fn custom(file_path:&str,compiler:Vec<&str>,rs:&str){
    main(file_path,compiler,rs);
}
pub fn main(file_path:&str, compiler:Vec<&str>, rs:&str){
    let mut compiler = compiler.iter().map(|&s| s.to_ascii_lowercase()).collect::<Vec<String>>();
    let file_path = Path::new(file_path);
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let package = get_package(file_path.to_path_buf());
    let java_flag = file_name.contains(".java");
    compiler.push(file_path.to_str().unwrap().to_string());
    println!("Command: {:?}",  Command::new(compiler[0].clone())
        .args(compiler[1..].iter()));
    let mut compile_result = Command::new(compiler[0].clone())
        .args(compiler[1..].iter())
        .spawn()
        .expect("make sure compiler exists");

    let _ = compile_result.wait().expect("failed to wait on child");

    let class_name = remove_last_dot_suffix(file_name);
    let javap_cmd = Command::new("javap")
        .args(["-s", &format!("target/classes/{}/{}.class",package.replace(".","/"), class_name)])
        .stdout(Stdio::piped()) // 捕获标准输出
        .spawn()
        .expect("make sure javap exists");

    // 读取 javap 的输出
    let output = javap_cmd
        .stdout
        .expect("Failed to capture stdout");
    let mut output_string = String::new();
    let mut reader = io::BufReader::new(output);

    // 将输出读取到字符串中
    reader.read_to_string(&mut output_string).expect("Failed to read output");
    println!("{}", output_string);
    let funs = if java_flag{Fun::java(output_string)}else{Fun::kt(output_string)};
    let code = generate_code(package,file_name,funs);
    std::fs::write(rs, code).expect(&format!("can not create {}",rs));
}

pub fn generate_code(package:String,file_name:&str,funs:Vec<Fun>)->String{
    let class_name =format!("{}/{}",package.replace(".","/"),remove_last_dot_suffix(file_name));
    let mut methods_str = String::new();
    for fun in funs {
        let method_str = format!(r#"
          jni::NativeMethod {{
                name: JNIString::from("{}"),
                sig: JNIString::from("{}"),
                fn_ptr: {} as *mut _,
            }},
        "#,fun.name,fun.sig,fun.name);
        methods_str+=method_str.as_str();
    }
    format!(r#"
        #[no_mangle]
        pub extern "system" fn JNI_OnLoad(vm: jni::JavaVM, _reserved: *mut std::ffi::c_void) -> jni::sys::jint {{
            let mut env = vm.get_env().expect("Cannot get reference to the JNIEnv");
            let class_name = "{}"; // 类名
            let class = env.find_class(class_name).unwrap();

            let methods = [
                {}
            ];
             env.register_native_methods(class, &methods).expect("Failed to register native methods");
            jni::sys::JNI_VERSION_1_6
        }}
    "#,class_name,methods_str)
}

#[test]
fn code(){
    let funs:Vec<Fun> = vec![
        Fun { sig: "(Ljava/lang/String;)Ljava/lang/String;".to_string(), name: "greeting".to_string() },
        Fun { sig: "(Ljava/lang/String;)Ljava/lang/String;".to_string(), name: "add".to_string() }
    ];
    generate_code("top.pingfanh.jni".to_string(), "RustNative.kt", funs);
}

#[test]
fn test() {
    //let  java_path = "examples/RustNative.java"; // Kotlin 文件所在的路径
    let kt_path = "examples/RustNative.kt"; // Kotlin 文件所在的路径
    //java(java_path,"examples/java_jni.rs");
    kt(kt_path,"D:/BIN/kotlinc/bin/kotlinc.bat","examples/kt_jni.rs");
}

#[test]
fn decode(){
    let input = r#"
   Compiled from "RustNative.kt"
public final class RustNative {
  public static final RustNative$Companion Companion;
    descriptor: LRustNative$Companion;
  public RustNative();
    descriptor: ()V

  public final native java.lang.String greeting(java.lang.String);
    descriptor: (Ljava/lang/String;)Ljava/lang/String;

  public final native java.lang.String add(java.lang.String);
    descriptor: (Ljava/lang/String;)Ljava/lang/String;

  static {};
    descriptor: ()V
}
    "#;
    let lines = input.lines().map(|x| x.to_string()).collect::<Vec<String>>();
    let mut funs:Vec<Fun>=Vec::new();
    // 通过索引循环遍历每一行
    let mut index = 0;
    while index < lines.len() {
        let line = lines.get(index).unwrap();
        println!("第 {} 行: {}", index + 1, line);
        if line.contains("native") {
            println!("native in {}", line);
            let re = Regex::new(r"public\s+final\s+native\s+([\w\.]+)\s+(\w+)\s*\(").unwrap();
            for cap in re.captures_iter(line) {
                if let Some(matched_type) = cap.get(1) {
                    if let Some(matched_name) = cap.get(2) {
                        let name = matched_name.as_str().to_string();
                        let descriptor_line = lines.get(index+1).unwrap();
                        let sig = fun::get_descriptor(descriptor_line);
                        println!("函数名: {}", matched_name.as_str());
                        println!("{}",descriptor_line);
                        let get_descriptor = fun::get_descriptor(descriptor_line);
                        println!("函数签名|{}", get_descriptor);
                        let fun = Fun{
                            name,sig
                        };
                        funs.push(fun);
                        index+=1;
                    }
                }
            }
        }
        index+=1;
    }
    println!("funs {:?}",funs)
}
#[test]
fn package() {
    let kt_path = ""; // Kotlin 文件所在的路径
    let kt_file = "RustNative.kt";
    let package = &format!("{}{}", kt_path, kt_file);
    let content = String::from_utf8(std::fs::read(package).unwrap()).unwrap();
}