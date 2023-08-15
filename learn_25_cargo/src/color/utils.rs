use super::Color;

/// Mixes two colors together in equal parts to create a new color.
///
/// # Examples
///
/// ```
/// let red = learn_25_cargo::PrimaryColor {
///     red: 255,
///     green: 0,
///     blue: 0,
/// };
///
/// let yellow = learn_25_cargo::SecondaryColor {
///     red: 255,
///     green: 255,
///     blue: 0,
/// };
///
/// let orange = learn_25_cargo::mix(&red, &yellow);
///
/// assert_eq!(orange.0, 255);
///
/// assert_eq!(orange.1, 127);
///
/// assert_eq!(orange.2, 0);
///
/// ```
pub fn mix(c1: &impl Color, c2: &impl Color) -> (u8, u8, u8) {
    let (r1, g1, b1) = c1.get_color();
    let (r2, g2, b2) = c2.get_color();
    (
        ((r1 as u16 + r2 as u16) / 2) as u8,
        ((g1 as u16 + g2 as u16) / 2) as u8,
        ((b1 as u16 + b2 as u16) / 2) as u8,
    )
}