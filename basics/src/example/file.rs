use std::{fs};

pub fn read_file(path: &str) -> String {
    let res = fs::read_to_string(path);
    match res {
        Ok(res) => {
            res
        }
        Err(res) => {
            res.to_string()
        }
    }
}

pub fn write_file(path: &str, content: &str) -> String {
    let res = fs::write(path, content);
    match res {
        Err(res) => {
            res.to_string()
        }
        _ => {
            String::from("写入成功")
        }
    }
}

pub fn file_stat(path: &str) -> String {
    let res = fs::metadata(path);

    match res {
        Ok(res) => {
            format!("created: {:?}, type: {:?}, modified: {:?}, length: {:?}", res.created(), res.file_type(), res.modified(), res.len())
        }
        Err(e) => {
            String::from("读取文件信息错误")
        }
    }

    
}