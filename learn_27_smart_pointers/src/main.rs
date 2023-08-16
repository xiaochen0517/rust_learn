mod cons_list;

fn main() {
    test_stack_str();
    test_box_var();
    cons_list::test_cons_list_var();
    test_box_var_compare();
}

fn change_str(str: &str) {
    println!("str: {}", str);
}

fn test_stack_str() {
    println!("====== test_stack_str ======");
    let s1 = "hello";
    println!("s1: {}", s1);
    change_str(s1);
    println!("s1: {}", s1);
}

fn test_box_var() {
    println!("====== test_box_var ======");
    let b = Box::new(5);
    println!("b = {}", b);
}

fn test_box_var_compare() {
    println!("====== test_box_var_compare ======");
    let a = 5;
    let b = Box::new(10);
    assert_eq!(a, *b);
}
