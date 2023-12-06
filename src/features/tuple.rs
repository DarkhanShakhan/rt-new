use std::ops::{Add, Div, Mul, Neg, Sub};

use super::consts::EPSILON;

#[derive(Debug, Copy, Clone, Default, PartialOrd)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z }
    }
    pub fn as_str(&self) -> String {
        format!("{} {} {}\n", self.x as i64, self.y as i64, self.z as i64)
    }
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(&self, other: &Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
    }
}
impl Eq for Tuple {}

impl Add for Tuple {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Tuple::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Tuple {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Tuple::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut x = -self.x;
        if self.x == 0.0 {
            x = 0.0;
        }
        let mut y = -self.y;
        if self.y == 0.0 {
            y = 0.0;
        }
        let mut z = -self.z;
        if self.z == 0.0 {
            z = 0.0;
        }
        Tuple::new(x, y, z)
    }
}

impl Div<f64> for Tuple {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Mul for Tuple {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}
