fn main() {
    vector_use_new();
    vector_use_macro();
}

fn vector_use_new() {
    println!("====== vector_use_new ======");
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v: {:?}", v);
}

fn vector_use_macro() {
    println!("====== vector_use_macro ======");
    let v = vec![10, 20, 30];
    println!("v: {:?}", v);
}
