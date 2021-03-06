mod example;

use example::{
    multiply, 
    read_file, 
    write_file,
    reverse_tuple, 
    file_stat, 
    test_enum, 
    test_io,
    print_rand,
    guess_game
};

use std::collections::HashMap;

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
    impl People<'_> {
       fn new(name: &str, age: u32) -> People {
            People {
                name: name,
                age: age,
                hobby: ["兴趣1", "兴趣2", "兴趣3"],
            }
        }
        fn get_name(&self) -> &str {
            self.name
        }
    }
    // 元组结构体
    #[derive(Debug)]
    struct Tup(i32, char);

    let tup_instance = Tup(-123, 's');

    let ming = People::new("lili", 12);
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
    println!("获取的名字 {}", ming.get_name());
    println!("vec {:#?}", vec);
    println!("元组结构体 {:?}", tup_instance);
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

// 计算字符出现字数
fn calc_count() {
    let string = "abcdefasdasdawe";
    let mut res: HashMap<char, i32> = HashMap::new();
    for c in string.chars() {
        match res.get_mut(&c) {
            Option::None => {
                res.insert(c, 1);
            }
            Option::Some(v) => {
                *v += 1;
            }
        };
    }
    println!("{:?}", res);
}

fn calc_count_two() {
    let string = "abcdefasdasdawe";
    let mut res: HashMap<char, i32> = HashMap::new();
    for c in string.chars() {
        let counter = res.entry(c).or_insert(0);
        *counter += 1;
    }
    println!("{:?}", res);
}

fn main() {
    calc_count_two();
    // test_var();
    // test_type();
    // println!("Hello, Rust!");
    // // test: 打印 99表
    // multiply();
    // let tup = (222, "222");
    // println!("{:?}", reverse_tuple(tup));
    // test_loop();
    // println!("{:?}", read_file("src/test_file.txt"));
    // println!("文件写入：{}", write_file("src/test_file.txt", "\\n你好，Rust"));
    // println!("文件信息 {}", file_stat("src/test_file.txt"));
    // println!("测试SFTP");
    // test_enum();
    // print_rand();
    // test_io();
    // guess_game();
}
