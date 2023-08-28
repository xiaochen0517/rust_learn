use crate::draw::Draw;

pub struct TextField {
    text: String,
}

impl TextField {
    pub fn new(text: String) -> TextField {
        TextField {
            text,
        }
    }
}

impl Draw for TextField {
    fn draw(&self) {
        println!("{}", self.text);
    }
}