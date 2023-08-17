use std::fmt::{Debug, Display};
use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T: Display + Debug>(T);

impl<T: Display + Debug> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: Display + Debug> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        println!("run MyBox::deref()");
        &self.0
    }
}

impl<T: Display + Debug> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("run MyBox::drop() drop data: {:?}", self.0);
    }
}

fn show_i32(num: &i32) {
    println!("num = {}", num);
}

fn show_str(str: &str) {
    println!("str = {}", str);
}

pub fn test_cast_type() {
    println!("====== test_cast_type ======");
    let num = MyBox::new(5);
    show_i32(&num);
    let str = MyBox::new("hello");
    show_str(&(*(&(*str))));
}

pub fn test_my_box() {
    println!("====== test_my_box ======");
    let a = 5;
    let b = MyBox::new(5);
    assert_eq!(a, *b);
    println!("a = {}, b = {}", a, *b);
}

pub fn test_my_box_drop() {
    println!("====== test_my_box_drop ======");
    {
        let num = MyBox::new(5);
        println!("num = {:?}", num);
        println!("drop before ------");
    }
    println!("drop after ------");
}

pub fn test_manually_drop() {
    println!("====== test_manually_drop ======");
    {
        let num = MyBox::new(5);
        println!("num = {:?}", num);
        println!("drop before ------");
        drop(num);
        println!("drop after ------");
        // println!("num = {:?}", num); // error[E0382]: borrow of moved value: `num`
    }
    println!("scope end ------");
}