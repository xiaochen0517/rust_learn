fn main() {
    test_lifetime();
    none_used_lifetime();
    test_struct_lifetime();
    test_struct_method();
    test_static_lifetime();
}

fn test_lifetime() {
    println!("====== test_lifetime ======");
    let str1 = String::from("abcd");
    let str2 = "xyz";
    let result = test_function(&str1, &str2);
    println!("The longest string is {}", result);
}

fn test_function<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn none_used_lifetime() {
    println!("====== none_used_lifetime ======");
    let str1: String = String::from("abcd");
    let result: &str;
    {
        let str2: String = String::from("xyz");
        result = none_used_function(&str1, &str2);
    }
    println!("The longest string is {}", result);
}

fn none_used_function<'a>(str1: &'a str, str2: &str) -> &'a str {
    println!("str1: {}", str1);
    println!("str2: {}", str2);
    str1
}

fn test_struct_lifetime() {
    println!("====== test_struct_lifetime ======");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let important_excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("important : {:?}", important_excerpt);
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn test_struct_method() {
    println!("====== test_struct_method ======");
    let struct_instance = MyStruct {
        name: String::from("zhangsan"),
        age: 18,
    };
    let name;
    let test_var;
    {
        test_var = 2u8;
        name = struct_instance.get_name(&test_var);
    }
    println!("name: {}", name);
    println!("test var: {}", test_var);
}

struct MyStruct {
    name: String,
    age: u32,
}

impl MyStruct {
    fn get_name<'a, 'b>(&'a self, num: &'b u8) -> &'b str
        where 'a: 'b,
    {
        println!("num: {}", num);
        &self.name
    }
}

fn test_static_lifetime() {
    println!("======= test_static_lifetime ======");
}