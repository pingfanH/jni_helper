
extern crate regex;
use regex::Regex;
#[derive(Debug)]
pub struct Fun{
    pub(crate) sig:String,
    pub(crate) name:String
}
impl Fun {
    pub fn new(input:String) -> Vec<Fun> {
        let mut lines = input.lines().map(|x| x.to_string()).collect::<Vec<String>>();
        let mut funs:Vec<Fun>=Vec::new();
        // 通过索引循环遍历每一行
        let mut index = 0;
        while index < lines.len() {
            let line = lines.get(index).unwrap();
            if line.contains("native") {
                let re = Regex::new(r"public\s+final\s+native\s+([\w\.]+)\s+(\w+)\s*\(").unwrap();
                for cap in re.captures_iter(line) {
                    if let Some(matched_type) = cap.get(1) {
                        if let Some(matched_name) = cap.get(2) {
                            let name = matched_name.as_str().to_string();
                            let descriptor_line = lines.get(index+1).unwrap();
                            let sig = get_descriptor(descriptor_line);
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
        funs
    }
}
pub(crate) fn get_descriptor(str:&str) -> String {
    str.replace("descriptor:","").trim().to_string()
}