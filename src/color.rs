use std::ops;

#[derive(Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
}

impl ops::Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color{ r: self.r + other.r, g: self.g + other.g, b: self.b + other.b }
    }
}

impl ops::Sub for Color {
    type Output = Color;
    fn sub(self, other: Color) -> Color {
        Color{ r: self.r - other.r, g: self.g - other.g, b: self.b - other.b }
    }
}

impl ops::Mul for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color{ r: self.r * other.r, g: self.g * other.g, b: self.b * other.b }
    }
}

impl ops::Mul<u8> for Color {
    type Output = Color;
    fn mul(self, value: u8) -> Color {
        Color{ r: self.r * value, g: self.g * value, b: self.b * value }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;
    fn mul(self, value: f64) -> Color {
        Color{ r: ((self.r as f64) * value) as u8, g: ((self.g as f64) * value) as u8, b: ((self.b as f64) * value) as u8 }
    }
}

impl ops::Div for Color {
    type Output = Color;
    fn div(self, other: Color) -> Color {
        Color{ r: self.r / other.r, g: self.g / other.g, b: self.b / other.b }
    }
}

impl ops::Div<u8> for Color {
    type Output = Color;
    fn div(self, value: u8) -> Color {
        Color{ r: self.r / value, g: self.g / value, b: self.b / value }
    }
}