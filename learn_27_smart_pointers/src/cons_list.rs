#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct ConsListIterator<'a> {
    current: Option<&'a List>,
}

/// 实现List的迭代器
impl<'a> Iterator for ConsListIterator<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(&List::Cons(ref value, ref next)) => {
                self.current = Some(next);
                Some(value)
            }
            Some(&List::Nil) => {
                self.current = None;
                None
            }
            None => None,
        }
    }
}

impl List {
    fn iter(&self) -> ConsListIterator {
        ConsListIterator {
            current: Some(self),
        }
    }
}

pub fn test_cons_list_var() {
    println!("====== test_cons_list_var ======");
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("list: {:?}", list);
    for item in list.iter() {
        println!("item: {}", item);
    }
}