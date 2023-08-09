use std::error::Error;
use std::ffi::NulError;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};
use std::net::IpAddr;

fn main() -> Result<(), Box<dyn Error>> {
    // use_panic();
    // code_panic();
    // open_file_maybe_error();
    test_read_user_func();
    test_read_user_func_1();
    panic_or_not();

    Ok(())
}

fn use_panic() {
    panic!("crash and burn");
}

fn code_panic() {
    println!("====== code_panic ======");
    let v = vec![1, 2, 3];
    v[99];
}

fn open_file_maybe_error() {
    const FILE_PATH: &'static str = "test.txt";
    let file_result = File::open(FILE_PATH);
    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(FILE_PATH) {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        }
    };
    println!("file: {:?}", file);
}

fn test_read_user_func() {
    println!("====== test_read_user_func ======");
    let username_result = read_user_from_file();
    match username_result {
        Ok(username) => {
            println!("username: {}", username);
        }
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };
}

fn read_user_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("username.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn test_read_user_func_1() {
    println!("====== test_read_user_func_1 ======");
    let username_result = read_user_from_file_with_symbol();
    match username_result {
        Ok(username) => {
            println!("username: {}", username);
        }
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };
}

fn read_user_from_file_with_symbol() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn panic_or_not() {
    println!("====== panic_or_not ======");
    let home: IpAddr = "127.0.0.1".parse().expect("Invalid IP address");
    println!("home: {:?}", home);
}
