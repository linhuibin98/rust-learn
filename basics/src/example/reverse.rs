pub fn reverse_tuple(pair: (i32, &str)) -> (&str, i32) {
    let (num, string) = pair;
    (string, num)
}