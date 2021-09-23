// 变量声明
fn test_var() {
    let num: i32 = 123;
    let mut num2: i32 = 12;
    print!("add value {}", num + num2);
    num2 = 24;
    println!("num2 value {}", num2);
    const CONST_VAR: &str = "测试常量";
    println!("{}", CONST_VAR);
    let num: i32 = 666;
    println!("{}", num);
}

// 数据类型
fn test_type() {
    
}

fn main() {
    test_var();
    println!("Hello, Rust!");
}
