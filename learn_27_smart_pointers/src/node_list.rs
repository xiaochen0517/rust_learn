use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
#[allow(dead_code)]
struct Node<T> {
    parent: RefCell<Weak<Node<T>>>,
    value: T,
    next: RefCell<Vec<Rc<Node<T>>>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_list() {
        let leaf = Rc::new(Node {
            parent: RefCell::new(Weak::new()),
            value: 3,
            next: RefCell::new(vec![]),
        });

        println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
        println!("------");
        {
            let branch = Rc::new(Node {
                parent: RefCell::new(Weak::new()),
                value: 5,
                next: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
            println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
            println!("------");
            println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
            println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

            println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        }
        println!("------");
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
}
