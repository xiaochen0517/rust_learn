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