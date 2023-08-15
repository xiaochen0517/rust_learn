pub(crate) mod background_color;
pub(crate) mod utils;

/// A trait for colors.
///
/// This trait is implemented for the two primary colors, red and blue.
pub trait Color {

    /// Returns the color as a tuple of `u8` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use learn_25_cargo::Color;
    /// let red = learn_25_cargo::PrimaryColor {
    ///     red: 255,
    ///     green: 0,
    ///     blue: 0,
    /// };
    ///
    /// assert_eq!(red.get_color(), (255, 0, 0));
    /// ```
    fn get_color(&self) -> (u8, u8, u8);
}