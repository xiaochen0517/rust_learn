fn main() {
    test_rectangle_method();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn get_area(&self) -> u32 {
        self.width * self.height
    }
}

fn test_rectangle_method() {
    println!("======test_rectangle_method======");
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect: {:#?}", rect);
    println!("rect.get_area(): {}", rect.get_area());
}
