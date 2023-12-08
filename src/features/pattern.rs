use crate::features::EPSILON;

use super::{
    color::Color,
    consts::{BLACK, WHITE},
    matrice::Matrice,
    object::Object,
    point::Point,
};
use float_cmp::approx_eq;
#[derive(Default, Debug, PartialEq, Clone, PartialOrd)]
pub struct Pattern {
    pattern_type: PatternType,
    transformation: Matrice,
    transformation_inverse: Matrice,
}

impl Pattern {
    pub fn new(pattern_type: PatternType, transformation: Matrice) -> Pattern {
        let mut output = Pattern {
            pattern_type,
            transformation,
            ..Default::default()
        };
        output.transformation_inverse = output.transformation.inverse();
        output
    }
    pub fn at(&self, object: &Object, point: &Point) -> Color {
        let object_point = object.transformation_inverse() * point;
        let pattern_point = &self.transformation_inverse * &object_point;
        self.pattern_type.at(&pattern_point)
    }
    pub fn ring(c1: Color, c2: Color) -> Pattern {
        Pattern {
            pattern_type: PatternType::Ring(c1, c2),
            ..Default::default()
        }
    }
    pub fn stripe(c1: Color, c2: Color) -> Pattern {
        Pattern {
            pattern_type: PatternType::Stripe(c1, c2),
            ..Default::default()
        }
    }
    pub fn checker(c1: Color, c2: Color) -> Pattern {
        Pattern {
            pattern_type: PatternType::Checker(c1, c2),
            ..Default::default()
        }
    }
    pub fn gradient(from: Color, to: Color) -> Pattern {
        Pattern {
            pattern_type: PatternType::Gradient(from, to),
            ..Default::default()
        }
    }
    pub fn test() -> Pattern {
        Pattern {
            pattern_type: PatternType::Test,
            ..Default::default()
        }
    }
    pub fn set_transformation(&mut self, transformation: Matrice) {
        self.transformation = transformation;
        self.transformation_inverse = self.transformation.inverse();
    }
}
#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum PatternType {
    Ring(Color, Color),
    Stripe(Color, Color),
    Gradient(Color, Color),
    Checker(Color, Color),
    Test,
}

impl Default for PatternType {
    fn default() -> Self {
        PatternType::Stripe(BLACK, WHITE)
    }
}

impl PatternType {
    pub fn at(&self, point: &Point) -> Color {
        match self {
            PatternType::Ring(c1, c2) => at_ring(c1, c2, point),
            PatternType::Stripe(c1, c2) => at_stripe(c1, c2, point),
            PatternType::Gradient(from, to) => at_gradient(from, to, point),
            PatternType::Checker(c1, c2) => at_checker(c1, c2, point),
            PatternType::Test => at_test(point),
        }
    }
}

fn at_ring(c1: &Color, c2: &Color, point: &Point) -> Color {
    if (point.position.x * point.position.x + point.position.z * point.position.z).sqrt() as i32 % 2
        == 0
    {
        return *c1;
    }
    *c2
}

fn at_stripe(c1: &Color, c2: &Color, point: &Point) -> Color {
    if point.position.x.floor() as i32 % 2 == 0 {
        return *c1;
    }
    *c2
}

fn at_gradient(from: &Color, to: &Color, point: &Point) -> Color {
    let distance = *to - *from;
    let fraction = point.position.x - point.position.x.floor();
    *from + distance * fraction
}

fn at_checker(c1: &Color, c2: &Color, point: &Point) -> Color {
    let sum = point.position.x.floor() + point.position.y.abs().floor() + point.position.z.floor();
    if approx_eq!(f64, sum % 2.0, 0.0, epsilon = EPSILON) {
        return *c1;
    }
    *c2
}

fn at_test(point: &Point) -> Color {
    Color::from(point.position)
}
