fn main() {
    test_basic_closure();
    test_struct_closure();
}

fn test_basic_closure() {
    println!("====== test_basic_closure ======");
    let x = 12;
    let add_x = |y: i32| x + y;
    let result = add_x(12);
    println!("result: {}", result);
}

struct Cacher<T, E>
    where
        T: Fn(E) -> E,
{
    calculation: T,
    value: Option<E>,
}

impl<T, E> Cacher<T, E>
    where
        T: Fn(E) -> E,
{
    // new() is a static method
    fn new(calculation: T) -> Cacher<T, E> {
        Cacher {
            calculation,
            value: None,
        }
    }

    // value() is a instance method
    fn value(&mut self, arg: E) -> &E {
        match self.value {
            Some(ref v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                self.value.as_ref().unwrap_or_else(|| panic!("value is None"))
            }
        }
    }
}

fn test_struct_closure() {
    println!("====== test_struct_closure ======");
    let mut cacher = Cacher::new(|x| x + 1);
    let v1 = cacher.value(1);
    println!("v1: {}", v1);
    let v2 = cacher.value(2);
    println!("v2: {}", v2);
    let mut string_cacher = Cacher::new(|mut str: String| {
        str.push_str(" world");
        str
    });
    let s = String::from("hello");
    let s1 = string_cacher.value(s);
    println!("s1: {}", s1);
    let s2 = String::from("hello hahaha");
    string_cacher.value = None;
    let s3 = string_cacher.value(s2);
    println!("s3: {}", s3);
}
