use super::base_util;

pub fn get_first_word(str: &String) -> &str {
    base_util::println(str);
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }
    &str[..]
}