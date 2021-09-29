use std::io;

pub fn test_io() {
    println!("请输入您的名称？");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
