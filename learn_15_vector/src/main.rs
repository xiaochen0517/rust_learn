fn main() {
    vector_use_new();
    vector_use_macro();
    vector_read();
    vector_read_option_option();
    vector_dy_element();
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

fn vector_read() {
    println!("====== vector_read ======");
    let mut v = vec![10, 20, 30];
    let element_1: &i32 = v.get(2).expect("获取到的值为空");
    println!("third: {}", element_1);
    v.push(40);
    let element_2: &i32 = v.get(3).expect("获取到的值为空");
    println!("third: {}", element_2);
}

fn vector_read_option_option() {
    println!("====== vector_read_option_option ======");
    let mut v: Vec<Option<i32>> = Vec::new();
    v.push(Some(1));
    v.push(Some(2));
    v.push(Some(3));
    v.push(None);
    println!("v: {:?}", v);
    let element_1: &Option<i32> = v.get(2).expect("获取到的值为空");
    println!("third: {:?}", element_1);
    let element_2: &Option<i32> = v.get(3).expect("获取到的值为空");
    println!("third: {:?}", element_2);
}

fn vector_dy_element() {
    println!("====== vector_dy_element ======");
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row: {:?}", row);
}
