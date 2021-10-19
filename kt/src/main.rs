extern crate clap;

use clap::{App, Arg};
use std::io;
use std::path::Path;
use std::process;
use std::fs::File;

use io::{Read, Write};

fn main() {
    let matches = App::new("kt")
        .version("1.0.0")
        .author("linhuibin")
        .about("a cmd program")
        .arg(
            Arg::with_name("FILE")
                .help("File to print.")
                .empty_values(false),
        )
        .get_matches();
    
    match matches.value_of("FILE") {
        Some(file) => {
            if Path::new(file).exists() {
                // expect();
                match File::open(file) {
                    Ok(mut f) => {
                        let mut data = String::new();
                        f.read_to_string(&mut data).expect("[kt Error] Unable to read the  file.");
                        // 获取全局 stdout 对象
                        let stdout = io::stdout();
                        // 将 handle 包装在缓冲区中
                        let mut handle = io::BufWriter::new(stdout);
                        match writeln!(handle, "{}", data) {
                            Ok(_) => {},
                            Err(err) => {
                                eprintln!("[kt Error] Unable to display the file contents. {:?}", err);
                                process::exit(1);
                            }
                        }
                    },
                    Err(_err) => {
                        eprintln!("[kt Error] File not found.");
                    }
                }
            } else {
                eprintln!("[kt Error] Need such file or directory.");
                process::exit(1);
            }
        },
        None => {
            eprintln!("[kt Error] Need such file or directory.");
            process::exit(1);
        }
    }
}
