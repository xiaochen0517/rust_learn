use super::Color;

/// Mixes two colors together in equal parts to create a new color.
pub struct PrimaryColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/// Mixes two colors together in equal parts to create a new color.
impl Color for PrimaryColor {
    fn get_color(&self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }
}

/// Mixes two colors together in equal parts to create a new color.
pub struct SecondaryColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/// Mixes two colors together in equal parts to create a new color.
impl Color for SecondaryColor {
    fn get_color(&self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }
}
