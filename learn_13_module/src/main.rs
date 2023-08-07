use learn_library_1::add;
use crate::tools::str_util;

pub mod tools;

fn main() {
    let result = add(2, 2);
    assert_eq!(result, 4);

    let str: String = String::from("Hello world!");
    let first_word: &str = str_util::get_first_word(&str);
    println!("first_word: {}", first_word);

    customer::eat_at_restaurant();
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("front_of_house --- hosting --- add_to_waitlist");
        }
    }

    pub mod other {
        pub fn add_to_waitlist() {
            println!("front_of_house --- other --- add_to_waitlist");
        }
    }
}

mod back_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("back_of_house --- hosting --- add_to_waitlist");
        }
    }

    pub mod other {
        pub fn add_to_waitlist() {
            println!("back_of_house --- other --- add_to_waitlist");
        }
    }
}

mod customer {
    use crate::front_of_house::hosting::add_to_waitlist as front_hosting_add_to_waitlist;
    use crate::front_of_house::other::add_to_waitlist as front_other_add_to_waitlist;

    pub fn eat_at_restaurant() {
        front_hosting_add_to_waitlist();
        front_other_add_to_waitlist();
        crate::back_of_house::hosting::add_to_waitlist();
    }
}
