pub fn test_enum() {
    #[derive(Debug)]
    enum Code {
        Success(u16),
        Error(u16)
    }
    println!("{:?}", Code::Success(200));
}