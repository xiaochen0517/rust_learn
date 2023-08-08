extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    concat_string();
    format_string();
    chinese_string_1();
    chinese_string_2();
}

fn concat_string() {
    println!("====== concat_string ======");
    let s1 = String::from("foo");
    let mut s2 = String::from("bar");
    let s3 = s1.clone() + &*s2;
    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
    s2.clear();
    println!("s2: {}", s2);
    println!("s3: {}", s3);
}

fn format_string() {
    println!("====== format_string ======");
    let mut s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
    println!("s4: {}", s4);
    println!("------ clear s1 ------");
    s1.clear();
    println!("s1: {}", s1);
    println!("s4: {}", s4);
}

fn chinese_string_1() {
    println!("====== chinese_string_1 ======");
    let s1 = String::from("你好，世界。Hello, World.");
    let s2 = &s1[0..3];
    println!("s1: {}", s2);
}

fn chinese_string_2() {
    println!("====== chinese_string_2 ======");
    let mut s1 = String::from("你好，世界。Hello, World.");
    println!("s1: {}", s1);
    let mut graphemes: Vec<&str> = UnicodeSegmentation::graphemes(s1.as_str(), true).collect::<Vec<&str>>();
    let mut slice_1_arr = &graphemes[0..2];
    // 将array转为字符串
    let mut slice_1 = slice_1_arr.join("");
    println!("slice_1: {}", slice_1);
    let mut slice_2_arr = &graphemes[4..8];
    // 将array转为字符串
    let mut slice_2 = slice_2_arr.join("");
    println!("slice_2: {}", slice_2);
    slice_1.push_str(slice_2.as_str());
    println!("slice_1: {}", slice_1);
    println!("s1: {}", s1);
}
