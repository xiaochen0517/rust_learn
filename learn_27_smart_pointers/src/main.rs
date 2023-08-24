mod cons_list;
mod my_box;
mod rc_list;
mod limit_tracker;

fn main() {
    test_stack_str();
    test_box_var();
    cons_list::test_cons_list_var();
    test_box_var_compare();
    my_box::test_my_box();
    my_box::test_cast_type();
    my_box::test_my_box_drop();
    my_box::test_manually_drop();
    rc_list::test_basic_test_rc();
    test_mutiple_ownership();
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
    let b = Box::new(5);
    assert_eq!(a, *b);
    println!("a = {}, b = {}", a, b);
}

#[derive(Debug)]
enum List {
    Item(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Item, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn test_mutiple_ownership() {
    println!("====== test_mutiple_ownership ======");
    let value = Rc::new(RefCell::new(5));
    let value1 = Rc::new(RefCell::new(123));
    let a = Rc::new(Item(Rc::clone(&value),
                         Rc::new(Item(Rc::clone(&value1),
                                      Rc::new(Nil)))));
    let b = Item(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Item(Rc::new(RefCell::new(4)), Rc::clone(&a));
    println!("------ no change ------");
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    println!("------ change ------");
    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    println!("------ change again ------");

    match *a {
        Item(ref value, ref next) => {
            match *next.clone() {
                Item(ref value1, ref next1) => {
                    *value1.borrow_mut() += 10;
                }
                Nil => println!("End of list ..."),
            }
        }
        Nil => println!("End of list"),
    }

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
