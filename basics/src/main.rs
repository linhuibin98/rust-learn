mod example;

use crate::example::reverse::reverse_tuple;

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
    // 数组
    let arr = [8; 5];
    // 数组 slice
    let arr_slice = &arr[0..=1];
    // 元组
    let tup = (1, "你好", 2, 3, "Rust");
    // 结构体
    struct People<'a> {
        name: &'a str,
        age: u32,
        hobby: [&'a str; 3],
    }

    let ming = People {
        name: "sdfdf",
        age: 18,
        hobby: ["吃饭", "睡觉", "打豆豆"],
    };
    // 向量（动态数组）
    let mut vec = Vec::from(ming.hobby);
    vec.push("摸鱼");

    println!("arr {:?}", arr);
    println!("arr slice {:?}", arr_slice);
    println!("tup {:?}", tup);
    println!("tup {} {}", tup.1, tup.4);
    println!(
        "My name is {}, age {}, hobby has {:?}",
        ming.name, ming.age, ming.hobby
    );
    println!("vec {:#?}", vec);
}

// test: 打印 99表
fn print_nine() {
    println!("打印99乘法表开始");
    let mut i: u32 = 1;
    let mut j: u32 = 1;
    let mut res = String::new();
    while i <= 9 {
        if i == j {
            res.push_str(&format!("{} * {} = {};\n", i, j, i * j));
            i += 1;
            j = 1;
        } else {
            res.push_str(&format!("{} * {} = {}; ", i, j, i * j));
            j += 1;
        }
    }
    println!("{}", res);
}

// loop
fn test_loop() {
    println!("遍历数组");
    let arr = ['h', 'e', 'l', 'l', 'o'];
    for v in arr.iter() {
        println!("{}", v);
    }
    println!("遍历字符串");
    for c in "world".chars() {
        println!("{}", c);
    }
}

fn main() {
    test_var();
    test_type();
    println!("Hello, Rust!");
    print_nine();
    let tup = (222, "222");
    println!("{:?}", reverse_tuple(tup));
    test_loop();
}
