use std::{fs};

pub fn read_file(path: &str) -> String {
    let res = fs::read(path);
    match res {
        Ok(res) => {
            String::from_utf8_lossy(&res).parse().unwrap()
        }
        Err(res) => {
            res.to_string()
        }
        _ => {
            String::from("错误")
        }
    }
}