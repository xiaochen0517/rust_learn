use crate::tools::str_util;

pub mod tools;

fn main() {
    let str = String::from("Hello world!");
    let first_word = str_util::get_first_word(&str);
    println!("first_word: {}", first_word);
    A::a_func();
}

mod A {
    use crate::B;

    pub fn a_func() {
        println!("a_func");
        B::b_func();
    }

    pub mod A_A {
        pub fn a_a_func() {
            println!("a_a_func");
        }
    }
}

mod B {
    use crate::A;

    pub(crate) fn b_func() {
        println!("b_func");
        A::A_A::a_a_func();
    }
}
