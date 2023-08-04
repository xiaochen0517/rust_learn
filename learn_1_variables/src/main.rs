fn main() {
    not_mut_variable();
    can_mut_variable();
    const_variable();
    shadow_variable();
    shadow_different_type_variable();
}

fn not_mut_variable() {
    println!("======not_mut_variable======");
    let num: u32;
    num = 10;
    println!("num = {}", num);
    // num = 100; // error[E0384]: cannot assign twice to immutable variable `num`
    println!("num = {}", num);
}

fn can_mut_variable() {
    println!("======can_mut_variable======");
    let mut num: u32 = 10;
    println!("num = {}", num);
    num = 100;
    println!("num = {}", num);
}

fn const_variable() {
    println!("======const_variable======");
    const MAX_NUM: u32 = 100;
    println!("MAX_NUM = {}", MAX_NUM);
    // MAX_NUM = 1000; // error[E0070]: invalid left-hand side expression
    println!("MAX_NUM = {}", MAX_NUM);
}

fn shadow_variable() {
    println!("======shadow_variable======");
    let num: u32 = 10;
    println!("shadow before num = {}", num);
    {
        let num: u32 = 100;
        println!("shadowed num = {}", num);
    }
    println!("shadow after num = {}", num);
}

fn shadow_different_type_variable() {
    println!("======shadow_different_type_variable======");
    let num: u32 = 10;
    println!("shadow before num = {}", num);
    {
        let num: &str = "this is a string";
        println!("shadowed num = {}", num);
    }
    println!("shadow after num = {}", num);
}
