use std::fmt::Display;

fn main() {
    test_get_bigger_number();
    test_point_struct();
    test_persion_trait();
    test_return_quote();
}

fn test_get_bigger_number() {
    println!("====== test_get_bigger_number ======");
    let list = vec![1, 2, 3, 4, 5];
    let bigger = get_bigger_number(&list);
    println!("bigger: {}", bigger);
}

fn get_bigger_number<T: PartialOrd>(list: &[T]) -> &T {
    let mut bigger = &list[0];
    for item in list {
        if item > bigger {
            bigger = item;
        }
    }
    bigger
}

fn test_point_struct() {
    println!("====== test_point_struct ======");
    println!("------ use new ------");
    let mut point = Point::new(1, 2);
    point.print();
    println!("------ use set ------");
    point.set_x(3);
    point.set_y(4);
    point.print();
    println!("------ use get ------");
    println!("x: {}, y: {}", point.get_x(), point.get_y());
    println!("------ use distance_from_origin ------");
    let point = Point::new(3.0, 4.0);
    println!("distance_from_origin: {}", point.distance_from_origin());
}

struct Point<X: Display, Y: Display> {
    x: X,
    y: Y,
}

impl<X: Display, Y: Display> Point<X, Y> {
    fn new(x: X, y: Y) -> Self {
        Point { x, y }
    }

    fn get_x(&self) -> &X {
        &self.x
    }

    fn get_y(&self) -> &Y {
        &self.y
    }

    fn set_x(&mut self, x: X) {
        self.x = x;
    }

    fn set_y(&mut self, y: Y) {
        self.y = y;
    }

    fn print(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn test_persion_trait() {
    println!("====== test_persion_trait ======");
    let student = Student::new("Tom");
    some_person_speak(&student, "hello");
    let food = String::from("apple");
    some_person_eat(&student, &food);
    some_person_sleep(&student);
}

fn some_person_speak(person: &impl Person, word: &str) {
    person.speak(word)
}

fn some_person_eat(person: &impl Person, food: &String) {
    person.eat(food)
}

fn some_person_sleep(person: &impl Person) {
    person.sleep()
}

pub trait Speak {
    fn speak(&self, word: &str);
}

pub trait Eat {
    fn eat(&self, food: &String);
}

pub trait Sleep {
    fn sleep(&self);
}

pub trait Person: Speak + Eat + Sleep {}

pub struct Student {
    name: String,
}

impl Student {
    pub fn new(name: &str) -> Self {
        Student {
            name: name.to_string(),
        }
    }
}

impl Speak for Student {
    fn speak(&self, word: &str) {
        println!("{} speak: {}", self.name, word);
    }
}

impl Eat for Student {
    fn eat(&self, food: &String) {
        println!("{} eat {}", self.name, food);
    }
}

impl Sleep for Student {
    fn sleep(&self) {
        println!("{} sleep", self.name);
    }
}

impl Person for Student {}

fn test_return_quote() {
    println!("====== test_return_quote ======");
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let result = longest(s1.as_str(), s2.as_str());
    println!("result: {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn test_lifetime() {
    println!("====== test_lifetime ======");
    let result: &String;
    let s1 = String::from("hello");
    let s2 = String::from("world");
    {
        result = get_result(&s1, &s2);
    }
    println!("result: {}", result);
}

fn get_result<'a>(str: &'a String, result: &'a String) -> &'a String {
    result
}