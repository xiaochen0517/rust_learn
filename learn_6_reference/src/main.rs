fn main() {
    test_reference();
    test_reference_change_value();
    test_reference_mutible();
    test_data_race();
    test_mutiable_reference();
}

fn test_reference() {
    println!("======test_reference======");
    let str: String = String::from("hello");
    let len: usize = test_reference_1(&str);
    println!("str = {}", str);
    println!("len = {}", len);
}

fn test_reference_1(str: &String) -> usize {
    println!("======change_reference_value======");
    str.len()
}

fn test_reference_change_value() {
    println!("======test_reference_change_value======");
    let mut str: String = String::from("hello");
    test_reference_2(&mut str);
    println!("str = {}", str);
}

fn test_reference_2(str: &mut String) {
    str.push_str(" world");
}

fn test_reference_mutible() {
    println!("======test_reference_mutible======");
    let mut str: String = String::from("hello");
    test_1(&mut str);
    println!("str = {}", str);
}

fn test_1(str: &mut String) {
    str.push_str(" world");
    test_2(str);
}

fn test_2(str: &mut String) {
    str.push_str(" world");
}

fn test_data_race() {
    println!("======test_data_race======");
    let mut str: String = String::from("hello");
    let str_1: &mut String = &mut str;
    println!("str_1 = {}", str_1);
    let str_2: &mut String = &mut str;
    println!("str_2 = {}", str_2);
}

fn test_mutiable_reference() {
    println!("======test_mutiable_reference======");
    let str: String = String::from("hello");
    let str_1: &String = &str;
    let str_2: &String = &str;
    // let str_3: &mut String = &mut str; // error[E0502]: cannot borrow `str` as mutable because it is also borrowed as immutable
    println!("str_1 = {}", str_1);
    println!("str_2 = {}", str_2);
    // println!("str_3 = {}", str_3); // error[E0502]: cannot borrow `str` as mutable because it is also borrowed as immutable
}
