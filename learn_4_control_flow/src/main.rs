fn main() {
    test_if();
    test_loop();
    test_break_label();
    test_for_array();
}

fn test_if() {
    let num = if true {
        10
    } else {
        100
    };
    println!("num = {}", num);
}

fn test_loop() {
    let mut counter = 0;
    let loop_result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop_result = {}", loop_result);
}

fn test_break_label() {
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}

fn test_for_array() {
    let array = [10, 20, 30, 40, 50];
    for element in array {
        println!("the value is: {}", element);
    }
}
