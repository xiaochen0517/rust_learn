use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

fn main() {
    // use_panic();
    // code_panic();
    // open_file_maybe_error();
    test_read_user_func();
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
