use learn_13_module::learn_13_module_func;
use learn_13_module::tools::str_util;

fn main() {
    learn_13_module_func();
    let str: String = String::from("Hello world!");
    let first_word: &str = str_util::get_first_word(&str);
    println!("first_word: {}", first_word);
}