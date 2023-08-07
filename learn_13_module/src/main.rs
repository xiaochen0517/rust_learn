use crate::tools::str_util;

pub mod tools;

fn main() {
    let str = String::from("Hello world!");
    let first_word = str_util::get_first_word(&str);
    println!("first_word: {}", first_word);
    a::a_func();
}

mod a {
    use crate::b;

    pub fn a_func() {
        println!("a_func");
        b::b_func();
    }

    pub mod a_a {
        pub fn a_a_func() {
            println!("a_a_func");
        }
    }
}

mod b {
    use crate::a;

    pub(crate) fn b_func() {
        println!("b_func");
        a::a_a::a_a_func();
    }
}
