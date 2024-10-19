use std::path::{Path, PathBuf};

pub fn get_package(path:PathBuf) ->String{
    let content = String::from_utf8(std::fs::read(path).unwrap()).unwrap();
    for line in content.lines(){
        if line.contains("package "){
            let package =line.replace("package ","").replace(";","").trim().to_string();
            return package;
        }
    }
    "".to_string()
}
pub fn remove_last_dot_suffix(s: &str) -> &str {
    if let Some(pos) = s.rfind('.') {
        &s[..pos]
    } else {
        s
    }
}