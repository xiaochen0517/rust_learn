pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn add(a: &i32, b: &i32) -> i32 {
    a + b
}

pub fn return_10(num: i32) -> i32 {
    println!("num: {}", num);
    10
}

/*
    cargo test
*/

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_add() {
//         assert_eq!(add(&1, &2), 3);
//     }
//
//     #[test]
//     fn panic_test() {
//         panic!("Make this test fail");
//     }
//
//     #[test]
//     fn assert_fail_test() {
//         assert!(false);
//     }
//
//     #[test]
//     fn assert_success_use_return_function() {
//         let result_val = return_10(10);
//         assert_eq!(result_val, 10);
//     }
//
//     #[test]
//     #[ignore]
//     fn assert_fail_use_return_function() {
//         let result_val = return_10(10);
//         assert_eq!(result_val, 11);
//     }
// }