fn main() {
    test_basic_iterator();
    test_map_iterator();
    test_catch_env_closure();
    test_filter_iterator();
}

fn test_basic_iterator() {
    println!("====== test_basic_interator ======");
    let mut v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter_mut();
    let first_item = v1_iter.next().unwrap();
    println!("v1_iter.next() = {:?}", first_item);
    *first_item = 4;
    println!("v1 = {:?}", v1);
    let total_val: i32 = v1.iter().sum();
    println!("total_val = {}", total_val);
}

fn test_map_iterator() {
    println!("====== test_map_iterator ======");
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);
}

fn test_catch_env_closure() {
    println!("====== test_catch_env_closure ======");
    let str1 = String::from("hello");
    let str2 = String::from("world");
    let str3 = str1.clone() + &str2;
    println!("str1 = {}", str1);
    println!("str2 = {}", str2);
    println!("str3 = {}", str3);
}

fn test_filter_iterator() {
    println!("====== test_filter_iterator ======");
    let v1 = vec![1, 2, 3, 4, 5, 6];
    let num = 3i32;
    let v2: Vec<_> = v1.iter().filter(|x| **x > num).collect();
    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);
}
