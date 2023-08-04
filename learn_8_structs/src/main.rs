fn main() {
    test_user_struct();
    test_user_value();
    test_debug_println();
}


struct User<'a> {
    username: &'a mut String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn test_user_struct() {
    let user = User {
        username: &mut String::from("zhangsan"),
        email: String::from("xxx@qq.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("user.username: {}", user.username);
    println!("user.email: {}", user.email);
    println!("user.sign_in_count: {}", user.sign_in_count);
    println!("user.active: {}", user.active);
}

fn test_user_value() {
    println!("======test_user_value======");
    let mut username: String = String::from("zhangsan");
    let mut user1 = User {
        username: &mut username,
        email: String::from("xxx@qq.com"),
        sign_in_count: 1,
        active: true,
    };
    let mut user2 = User {
        // username: &mut String::from("lisi"),
        sign_in_count: 2,
        ..user1
    };
    // user1.username.push_str(" and lisi");
    // println!("user1.username: {}", user1.username);
    println!("user2.username: {}", user2.username);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn test_debug_println() {
    println!("======test_debug_println======");
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:#?}", rect1);
    // test dbg! macro
    let scale = 2;
    let rect2 = Rectangle { width: dbg!(30 * scale), height: 50 };
    dbg!(&rect2);
}
