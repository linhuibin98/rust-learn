use rand::Rng;
use std::{cmp, io};

const WELCOME: &str = "欢迎来到猜拳游戏";
const PLEASE: &str = "请输入你的数字1~100";

pub fn guess_game() {
    println!("{}", WELCOME);
    println!("{}", PLEASE);
    // 随机生成一个 1 - 100的数字
    let cop = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut val = String::new();
        io::stdin().read_line(&mut val).expect("系统发生错误");
        let val: u32 = match val.trim().parse() {
            Ok(v) => {
                if v > 100 {
                    println!("请输入1 - 100的数字");
                    continue;
                }
                v
            },
            Err(_) => {
                println!("请输入正确的数字");
                continue;
            }
        };
        match val.cmp(&cop) {
            cmp::Ordering::Greater => println!("太大了..."),
            cmp::Ordering::Equal => {
                println!("你赢了");
                break;
            }
            cmp::Ordering::Less => println!("太小了"),
        }
    }
}
