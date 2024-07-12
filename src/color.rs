use std::fmt;
use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: i32, g: i32, b: i32) -> Color {
        Color {
            r: r.clamp(0, 255) as u8,
            g: g.clamp(0, 255) as u8,
            b: b.clamp(0, 255) as u8,
        }
    }

    pub fn from_hex(hex: u32) -> Color {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Color { r, g, b }
    }

    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub fn set_color(&mut self, r: i32, g: i32, b: i32) {
        self.r = r.clamp(0, 255) as u8;
        self.g = g.clamp(0, 255) as u8;
        self.b = b.clamp(0, 255) as u8;
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        let r = self.r.saturating_add(other.r);
        let g = self.g.saturating_add(other.g);
        let b = self.b.saturating_add(other.b);
        Color { r, g, b }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, factor: f32) -> Color {
        let r = (self.r as f32 * factor).clamp(0.0, 255.0) as u8;
        let g = (self.g as f32 * factor).clamp(0.0, 255.0) as u8;
        let b = (self.b as f32 * factor).clamp(0.0, 255.0) as u8;
        Color { r, g, b }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color(r: {}, g: {}, b: {})", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let color = Color::new(255, 100, 50);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 100);
        assert_eq!(color.b, 50);
    }

    #[test]
    fn test_new_clamp() {
        let color = Color::new(300, -50, 256);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 255);
    }

    #[test]
    fn test_from_hex() {
        let color = Color::from_hex(0xFF6432);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 100);
        assert_eq!(color.b, 50);
    }

    #[test]
    fn test_to_hex() {
        let color = Color::new(255, 100, 50);
        assert_eq!(color.to_hex(), 0xFF6432);
    }

    #[test]
    fn test_add() {
        let color1 = Color::new(100, 150, 200);
        let color2 = Color::new(60, 100, 80);
        let result = color1 + color2;
        assert_eq!(result.r, 160);
        assert_eq!(result.g, 250);
        assert_eq!(result.b, 255); // saturating_add will cap this at 255
    }

    #[test]
    fn test_add_overflow() {
        let color1 = Color::new(200, 200, 200);
        let color2 = Color::new(100, 100, 100);
        let result = color1 + color2;
        assert_eq!(result.r, 255); // saturating_add will cap this at 255
        assert_eq!(result.g, 255); // saturating_add will cap this at 255
        assert_eq!(result.b, 255); // saturating_add will cap this at 255
    }

    #[test]
    fn test_mul() {
        let color = Color::new(100, 150, 200);
        let result = color * 1.5;
        assert_eq!(result.r, 150);
        assert_eq!(result.g, 225);
        assert_eq!(result.b, 255); // clamp will cap this at 255
    }

    #[test]
    fn test_mul_overflow() {
        let color = Color::new(200, 200, 200);
        let result = color * 1.5;
        assert_eq!(result.r, 255); // clamp will cap this at 255
        assert_eq!(result.g, 255); // clamp will cap this at 255
        assert_eq!(result.b, 255); // clamp will cap this at 255
    }

    #[test]
    fn test_mul_underflow() {
        let color = Color::new(100, 150, 200);
        let result = color * 0.5;
        assert_eq!(result.r, 50);
        assert_eq!(result.g, 75);
        assert_eq!(result.b, 100);
    }

    #[test]
    fn test_display() {
        let color = Color::new(100, 150, 200);
        assert_eq!(format!("{}", color), "Color(r: 100, g: 150, b: 200)");
    }
}
