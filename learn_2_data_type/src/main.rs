use std::str::FromStr;

use rust_decimal::Decimal;

fn main() {
    // decimal_max();
    float_add();
    test_overflow();
    test_decimal();
    test_div();
    test_char();
    test_tuple();
    test_array();
}

fn decimal_max() {
    println!("======decimal_max======");
    println!("------unsigned------");
    let u8_max: u8 = u8::MAX;
    println!("u8_max = {}", u8_max);
    let u16_max: u16 = u16::MAX;
    println!("u16_max = {}", u16_max);
    let u32_max: u32 = u32::MAX;
    println!("u32_max = {}", u32_max);
    let u64_max: u64 = u64::MAX;
    println!("u64_max = {}", u64_max);
    let u128_max: u128 = u128::MAX;
    println!("u128_max = {}", u128_max);
    let usize_max: usize = usize::MAX;
    println!("usize_max = {}", usize_max);
    println!("------signed------");
    let i8_max: i8 = i8::MAX;
    println!("i8_max = {}", i8_max);
    let i16_max: i16 = i16::MAX;
    println!("i16_max = {}", i16_max);
    let i32_max: i32 = i32::MAX;
    println!("i32_max = {}", i32_max);
    let i64_max: i64 = i64::MAX;
    println!("i64_max = {}", i64_max);
    let i128_max: i128 = i128::MAX;
    println!("i128_max = {}", i128_max);
    let isize_max: isize = isize::MAX;
    println!("isize_max = {}", isize_max);
    println!("------float------");
    let f32_max: f32 = f32::MAX;
    println!("f32_max = {}", f32_max);
    let f64_max: f64 = f64::MAX;
    println!("f64_max = {}", f64_max);
}

fn float_add() {
    let float_1: f64 = 0.1;
    let float_2: f64 = 0.2;
    let float_3: f64 = float_1 + float_2;
    println!("float_3 = {}", float_3);
}

fn test_overflow() {
    let u8_max: u8 = u8::MAX;
    println!("u8_max = {}", u8_max);
    let u8_overflow: (u8, bool) = u8_max.overflowing_add(1);
    println!("u8_overflow = {:?}", u8_overflow);
}

fn test_decimal() {
    let decimal_1: Decimal = Decimal::from_str("0.1").unwrap();
    let decimal_2: Decimal = Decimal::from_str("0.2").unwrap();
    let decimal_3: Decimal = decimal_1 + decimal_2;
    println!("decimal_3 = {}", decimal_3);
}

fn test_div() {
    let num_1: f64 = -5.0;
    let num_2: f64 = 3.0;
    let num_3: f64 = num_1 / num_2;
    println!("num_3 = {}", num_3);
}

fn test_char() {
    let char_1: char = '\u{1F645}';
    println!("char_1 = {}", char_1);
}

fn test_tuple() {
    let tuple_1: (i32, f64, char) = (1, 2.0, 'a');
    println!("tuple_1 index 0 = {}", tuple_1.0);
    println!("tuple_1 index 1 = {}", tuple_1.1);
    println!("tuple_1 index 2 = {}", tuple_1.2);
}

fn test_array() {
    let months_names: [&str; 12] = [
        "January", "February", "March", "April", "May", "June", "July", "August", "September",
        "October", "November", "December",
    ];
    println!("Please enter an integer:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: usize = input.trim().parse().unwrap();
    println!("The {}th month is {}", input, months_names[input - 1]);
}
