fn main() {
    concat_string();
}

fn concat_string() {
    println!("====== concat_string ======");
    let s1 = String::from("foo");
    let mut s2 = String::from("bar");
    let s3 = s1 + &*s2;
    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
    s2.clear();
    println!("s2: {}", s2);
    println!("s3: {}", s3);
}
