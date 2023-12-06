use std::ops::{Add, Mul, Neg, Sub};

use super::tuple::Tuple;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Color {
    pub rgb: Tuple,
}

impl Color {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Color::from(Tuple::new(x, y, z))
    }
    pub fn clamp(&self) -> Self {
        let mut x = self.rgb.x * 255.0;
        let mut y = self.rgb.y * 255.0;
        let mut z = self.rgb.z * 255.0;
        if x > 255.0 {
            x = 255.0;
        }
        if x < 0.0 {
            x = 0.0;
        }
        if y > 255.0 {
            y = 255.0;
        }
        if y < 0.0 {
            y = 0.0;
        }
        if z > 255.0 {
            z = 255.0;
        }
        if z < 0.0 {
            z = 0.0;
        }
        Color::from(Tuple::new(x, y, z))
    }
}

impl From<Tuple> for Color {
    fn from(value: Tuple) -> Self {
        Color { rgb: value }
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Color::from(self.rgb + rhs.rgb)
    }
}

impl Sub for Color {
    type Output = Color;
    fn sub(self, rhs: Self) -> Self::Output {
        Color::from(self.rgb - rhs.rgb)
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        Color::from(self.rgb * rhs)
    }
}

impl Mul for Color {
    type Output = Color;
    fn mul(self, rhs: Self) -> Self::Output {
        Color::from(self.rgb * rhs.rgb)
    }
}

impl Neg for Color {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Color::from(-self.rgb)
    }
}
