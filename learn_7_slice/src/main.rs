fn main() {
    test_slice();
    test_chinese_str_slice();
    test_slice_by_word();
}

fn test_slice() {
    println!("======test_slice======");
    let mut str: String = String::from("hello world");
    let len: usize = get_first_word_length(&str);
    println!("str = {}", str);
    println!("len = {}", len);
    // println!("------change_string------");
    // str.clear();
    // println!("str = {}", str);
    // println!("len = {}", len);

    let word: &str = &str[0..5];
    let word_len: usize = word.len();
    println!("word = {}", word);
    println!("word_len = {}", word_len);
    str.clear();
    println!("str = {}", str);
}

fn get_first_word_length(str: &String) -> usize {
    let bytes: &[u8] = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    str.len()
}

fn test_chinese_str_slice() {
    println!("======test_chinese_str_slice======");
    let str: String = String::from("你好，世界");
    println!("str length = {}", str.len());
    let str_slice: &str = &str[0..3];
    println!("str = {}", str);
    println!("str_slice = {}", str_slice);
}

fn test_slice_by_word() {
    println!("======test_slice_by_word======");
    let mut str: String = String::from("hello world");
    let first_word: &str = get_first_reference_word(&str);
    println!("first_word = {}", first_word);
    str.clear();
    println!("str = {}", str);
    // println!("first_word = {}", first_word); // error[E0502]: cannot borrow `str` as mutable because it is also borrowed as immutable
}

fn get_first_reference_word(str: &String) -> &str {
    let bytes: &[u8] = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }
    &str[..]
}
