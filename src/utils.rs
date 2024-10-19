
pub fn get_package(kt_path:&str,kt_file:&str)->String{
    let package = &format!("{}{}", kt_path, kt_file);
    let content = String::from_utf8(std::fs::read(package).unwrap()).unwrap();
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