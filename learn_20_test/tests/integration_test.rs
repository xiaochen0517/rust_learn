use learn_20_test::{add, add_two, return_10};

mod common;

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}

#[test]
fn test_add() {
    assert_eq!(add(&1, &2), 3);
}

#[test]
fn panic_test() {
    panic!("Make this test fail");
}

#[test]
fn assert_fail_test() {
    assert!(false);
}

#[test]
fn assert_success_use_return_function() {
    let result_val = return_10(10);
    assert_eq!(result_val, 10);
}

#[test]
#[ignore]
fn assert_fail_use_return_function() {
    let result_val = return_10(10);
    assert_eq!(result_val, 11);
}

#[test]
fn test_common_setup() {
    common::setup();
}