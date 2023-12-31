use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn run_sub_thread() {
    let sub_thread = thread::spawn(|| {
        println!("sub thread");
        for index in 0..20 {
            println!("sub thread index: {}", index);
            thread::sleep(Duration::from_millis(1));
        }
    });
    println!("main thread");
    for index in 0..10 {
        println!("main thread index: {}", index);
        thread::sleep(Duration::from_millis(1));
    }
    sub_thread.join().unwrap();
    println!("main thread end");
}

fn sub_thread_move() {
    println!("====== test_sub_thread_move ======");
    let v = vec![1, 2, 3];
    let sub_thread = thread::spawn(move || {
        println!("sub thread v: {:?}", v);
    });
    sub_thread.join().unwrap();
}

fn sub_thread_channel() {
    println!("====== test_sub_thread_channel ======");
    let (tx, rx) = mpsc::channel();
    let sub_thread = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    sub_thread.join().unwrap();
}

pub fn send_multiple_message() {
    println!("====== test_send_multiple_message ======");
    let (tx, rx) = mpsc::channel();
    let sub_thread = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("sub"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn multiple_send_message() {
    println!("====== test_multiple_send_message ======");
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let sub_thread_1 = thread::spawn(move || {
        let vals = vec![
            String::from("sub_thread_1 1"),
            String::from("sub_thread_1 2"),
            String::from("sub_thread_1 3"),
            String::from("sub_thread_1 4"),
            String::from("sub_thread_1 5"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(700));
        }
    });
    let sub_thread_2 = thread::spawn(move || {
        let vals = vec![
            String::from("sub_thread_2 1"),
            String::from("sub_thread_2 2"),
            String::from("sub_thread_2 3"),
            String::from("sub_thread_2 4"),
            String::from("sub_thread_2 5"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_sub_thread() {
        run_sub_thread();
    }

    #[test]
    fn test_sub_thread_move() {
        sub_thread_move();
    }

    #[test]
    fn test_sub_thread_channel() {
        sub_thread_channel();
    }
}