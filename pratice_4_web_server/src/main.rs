use std::{fs, thread};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use pratice_4_web_server::ThreadPool;

// 监听地址
static LISTEN_ADDRESS: &str = "127.0.0.1";
// 监听端口
static LISTEN_PORT: u32 = 7878u32;

fn main() {
    let listener = TcpListener::bind(format!("{}:{}", LISTEN_ADDRESS, LISTEN_PORT)).unwrap();
    println!("监听地址 http://{}:{}", LISTEN_ADDRESS, LISTEN_PORT);
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buffer_reader = BufReader::new(&mut stream);
    let request_line = buffer_reader.lines().next().unwrap().unwrap();
    println!("http_request: {:#?}", request_line);

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let html_content = fs::read_to_string(filename).unwrap();
    let length = html_content.len();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, html_content);
    stream.write_all(response.as_bytes()).unwrap();
}
