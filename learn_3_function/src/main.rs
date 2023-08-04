fn main() {
    test_expressions();
    let mut num = normal_function_has_return_value();
    println!("num in main = {}", num);
    change_number(&mut num);
    println!("num in main = {}", num);
}

fn test_expressions() {
    let exp = {
        let num = 10; // statement
        num + 100 // expression
    };
    println!("exp = {}", exp);
}

fn normal_function_has_return_value() -> u32 {
    println!("======normal_function_has_return_value======");
    let num: u32 = 10;
    println!("num = {}", num);
    num + 100 // return value
}

/// # Arguments
/// * `num` - a mutable reference to a u32
///
/// # Examples
/// ```
/// let mut num = 10;
/// change_number(&mut num);
///
/// assert_eq!(num, 100);
/// ```
fn change_number(num: &mut u32) {
    println!("======change_number======");
    println!("num = {}", num);
    *num = 100;
    println!("num = {}", num);
}
