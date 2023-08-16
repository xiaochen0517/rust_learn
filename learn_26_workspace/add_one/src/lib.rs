pub use rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn get_rand_num() -> i32 {
    rand::random::<i32>()
}

#[cfg(test)]
mod tests {
    // use super::add_one;
    use crate::add_one;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}