fn main() {
    // let tp = Type::A;
    let tp = Type::D; // other
    match tp {
        Type::A => {
            println!("A");
            println!("this is a");
        }
        Type::B => println!("B"),
        Type::C => println!("C"),
        _ => println!("other"),
    }
}

enum Type {
    A,
    B,
    C,
    D,
}
