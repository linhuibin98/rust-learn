pub fn multiply() {
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