//! # learn_25_cargo
//!
//! `learn_25_cargo` is a collection of utilities to make performing certain
//! calculations more convenient.
//!

mod color;
pub use color::Color;
pub use color::background_color::PrimaryColor;
pub use color::background_color::SecondaryColor;
pub use color::utils::mix;

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = learn_25_cargo::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}