use std::rc::Rc;

#[derive(Debug)]
enum RcList<T> {
    Cons(T, Rc<RcList<T>>),
    Nil,
}

use RcList::{Cons, Nil};

pub fn test_basic_test_rc() {
    println!("====== test_basic_test_rc ======");
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a rc count = {}", Rc::strong_count(&a));
    println!("a = {:?}", a);
    {
        let b = Cons(3, Rc::clone(&a));
        println!("a rc count = {}", Rc::strong_count(&a));
        println!("b = {:?}", b);
        {
            let c = Cons(4, Rc::clone(&a));
            println!("a rc count = {}", Rc::strong_count(&a));
            println!("c = {:?}", c);
        }
        println!("a rc count = {}", Rc::strong_count(&a));
    }
    println!("a rc count = {}", Rc::strong_count(&a));
}