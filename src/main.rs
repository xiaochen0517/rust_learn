fn main() {
    fibonacci_sequence(10);
}

fn fibonacci_sequence(n: u32) {
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut c: u32;
    println!("b = {}", b);
    for _i in 0..n {
        c = a + b;
        a = b;
        b = c;
        println!("b = {}", b);
    }
}
