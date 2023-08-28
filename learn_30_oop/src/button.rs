use crate::draw::Draw;

pub struct Button {
    pub width: i32,
    pub height: i32,
    pub text: String,
}

impl Button {
    pub fn new(width: i32, height: i32, text: String) -> Button {
        Button {
            width,
            height,
            text,
        }
    }
}

impl Draw for Button {
    fn draw(&self) {
        println!("-------------");
        println!("|{}|", self.text);
        println!("-------------");
    }
}