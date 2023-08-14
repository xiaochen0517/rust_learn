fn main() {
    test_basic_closure();
    test_struct_cacher_closure();
    test_system_type();
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

fn test_struct_cacher_closure() {
    println!("====== test_struct_cacher_closure ======");
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

fn test_system_type() {
    println!("====== test_system_type ======");
    // 获取系统类型，Windows、Linux、MacOS
    let os = std::env::consts::OS;
    println!("os: {}", os);
}

struct SaveClosure {
    calculation: Box<dyn Fn(i32) -> i32>,
    value: Option<i32>,
}

impl SaveClosure {
    fn new(calculation: Box<dyn Fn(i32) -> i32>) -> SaveClosure {
        SaveClosure {
            calculation,
            value: None,
        }
    }

    fn call_calculation(&self, x: i32) -> i32 {
        (self.calculation)(x)
    }
}

fn test_struct_closure() {
    println!("====== test_struct_closure ======");
    let mut num = &100i32;
    println!("num: {}", num);
    let mut save_closure = SaveClosure {
        calculation: Box::new(|x| {
            x + *num
        }),
        value: None,
    };
    let v1 = save_closure.call_calculation(1);
    println!("v1: {}", v1);
    num = &200;
    println!("num: {}", num);
    let v2 = save_closure.call_calculation(2);
    println!("v2: {}", v2);
}

fn test_main_closure() {
    println!("====== test_main_closure ======");
    let n1 = 100;
    let n2 = 200;
    let n3 = n1 + n2;
    let mut num = 100i32;
    println!("num: {}", num);
    let closure = |x| -> i32 { x + num };
    let v1 = closure(1);
    println!("v1: {}", v1);
    // num = 200; // error[E0384]: cannot assign twice to immutable variable `num`
    // println!("num: {}", num);
    let v2 = closure(2);
    println!("v2: {}", v2);
}
