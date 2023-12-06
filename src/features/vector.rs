use std::ops::{Add, Div, Mul, Neg, Sub};

use super::{point::Point, tuple::Tuple};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Vector {
    pub position: Tuple,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector {
            position: Tuple::new(x, y, z),
        }
    }
    pub fn magnitude(&self) -> f64 {
        self.position.magnitude()
    }

    pub fn normalize(&self) -> Self {
        Self::from(self.position / self.magnitude())
    }

    pub fn dot_product(&self, rhs: &Vector) -> f64 {
        self.position.dot(&rhs.position)
    }

    pub fn cross_product(&self, rhs: &Vector) -> Self {
        Self::from(Tuple::new(
            self.position.y * rhs.position.z - self.position.z * rhs.position.y,
            self.position.z * rhs.position.x - self.position.x * rhs.position.z,
            self.position.x * rhs.position.y - self.position.y * rhs.position.x,
        ))
    }
    pub fn reflect(&self, normal: &Self) -> Self {
        *self - *normal * 2.0 * self.dot_product(normal)
    }
}

impl From<Tuple> for Vector {
    fn from(value: Tuple) -> Self {
        Vector { position: value }
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector::from(self.position + rhs.position)
    }
}

impl Add<Point> for Vector {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point::from(self.position + rhs.position)
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector::from(self.position - rhs.position)
    }
}

impl Sub<Point> for Vector {
    type Output = Point;
    fn sub(self, rhs: Point) -> Self::Output {
        Point::from(self.position - rhs.position)
    }
}

impl Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vector::from(-self.position)
    }
}

impl Div<f64> for Vector {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Vector::from(self.position / rhs)
    }
}

impl Mul<f64> for Vector {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector::from(self.position * rhs)
    }
}

#[cfg(test)]
mod reflect_tests {
    use super::*;

    #[test]
    fn at_45() {
        let v = Vector::new(1.0, -1.0, 0.0);
        let n = Vector::new(0.0, 1.0, 0.0);
        let r = v.reflect(&n);
        assert_eq!(r, Vector::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn slanted_surface() {
        let v = Vector::new(0.0, -1.0, 0.0);
        let n = Vector::new(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
        let r = v.reflect(&n);
        assert_eq!(r, Vector::new(1.0, 0.0, 0.0));
    }
}
