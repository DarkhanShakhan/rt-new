use std::ops::{Add, Div, Neg, Sub};

use super::{tuple::Tuple, vector::Vector};

#[derive(PartialEq, Eq, Clone, Copy, Default, Debug)]
pub struct Point {
    pub position: Tuple,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point {
            position: Tuple::new(x, y, z),
        }
    }
}

impl Add for Point {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        Vector::from(self.position + rhs.position)
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Self::Output {
        Point::from(self.position + rhs.position)
    }
}

impl Sub for Point {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector::from(self.position - rhs.position)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;
    fn sub(self, rhs: Vector) -> Self::Output {
        Point::from(self.position - rhs.position)
    }
}

impl From<Tuple> for Point {
    fn from(value: Tuple) -> Self {
        Point { position: value }
    }
}

impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Point::from(-self.position)
    }
}

impl Div<f64> for Point {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Point::from(self.position / rhs)
    }
}
