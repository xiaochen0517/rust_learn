fn main() {
    test_lifetime();
}

fn test_lifetime() {
    println!("====== test_lifetime ======");
    let str1 = String::from("abcd");
    let str2 = "xyz";
    let result = test_function(&str1, &str2);
    println!("The longest string is {}", result);
}

fn test_function<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
