#[cfg(test)]
mod tests {
    use crate::add;

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
}