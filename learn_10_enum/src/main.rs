fn main() {
    test_enum();
    test_option();
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Write(s) => println!("writer message = {}", s),
            _ => println!("other message"),
        }
    }
}

#[derive(Debug)]
enum IPAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IPAddrKind {
    fn get_ip(&self) -> String {
        match self {
            IPAddrKind::V4(a, b, c, d) => format!("{}.{}.{}.{}", a, b, c, d),
            IPAddrKind::V6(s) => s.to_string(),
        }
    }
}

fn test_enum() {
    println!("======test_enum======");
    let four = IPAddrKind::V4(127, 0, 0, 1);
    let six = IPAddrKind::V6(String::from("::1"));
    println!("four: {:?}", four);
    println!("six: {:?}", six);
    println!("----------------------");
    println!("four.get_ip(): {}", four.get_ip());
    println!("six.get_ip(): {}", six.get_ip());
}

fn test_option() {
    println!("======test_option======");
    let some_number = Some(5);
    let some_string: Option<String> = Some(String::from("hello"));
    let absent_number: Option<i32> = None;
    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);
}
