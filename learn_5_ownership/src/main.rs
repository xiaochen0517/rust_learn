fn main() {
    test_string();
    string_clone();
    tuple_auto_copy();
    test_function_copy();
    test_function_move();
    test_function_move_tuple();
    test_reference();
    test_reference_change_value();
}

fn test_string() {
    println!("======test_string======");
    let str: String = String::from("hello");
    let str_1 = str;
    // println!("str = {}", str); // error[E0382]: borrow of moved value: `str`
    println!("str_1 = {}", str_1);
}

fn string_clone() {
    println!("======string_clone======");
    let str: String = String::from("hello");
    let str_1 = str.clone();
    println!("str = {}", str);
    println!("str_1 = {}", str_1);
}

fn tuple_auto_copy() {
    println!("======tuple_auto_copy======");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup_1 = tup;
    println!("tup = {:?}", tup);
    println!("tup_1 = {:?}", tup_1);
    println!("======tuple_auto_copy======");
    let tup: (i32, f64, String) = (500, 6.4, String::from("hello"));
    let tup_1 = tup;
    // println!("tup = {:?}", tup); // error[E0382]: borrow of moved value: `tup`
    println!("tup_1 = {:?}", tup_1);
}

fn test_function_copy() {
    println!("======test_function_copy======");
    let num: i32 = 10;
    let str: String = String::from("hello");
    test_function(num, str.clone());
    println!("outer num = {}", num);
    println!("outer str = {}", str);
}

fn test_function(mut num: i32, mut str: String) {
    num += 100;
    println!("func num = {}", num);
    str.push_str(" world");
    println!("func str = {}", str);
}

fn test_function_move() {
    println!("======test_function_move======");
    let str: String = String::from("hello");
    let str = test_function_1(str);
    println!("str = {}", str);
}

fn test_function_1(mut str: String) -> String {
    str.push_str(" world");
    str
}

fn test_function_move_tuple() {
    println!("======test_function_move_tuple======");
    let num: i32 = 10;
    let str: String = String::from("hello");
    let (num, str) = test_function_2(num, str);
    println!("num = {}", num);
    println!("str = {}", str);
}

fn test_function_2(num: i32, mut str: String) -> (i32, String) {
    str.push_str(" world");
    (num + 100, str)
}

fn test_reference() {
    println!("======test_reference======");
    let str: String = String::from("hello");
    let len: usize = test_reference_1(&str);
    println!("str = {}", str);
    println!("len = {}", len);
}

fn test_reference_1(str: &String) -> usize {
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
