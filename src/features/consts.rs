use super::{color::Color, tuple::Tuple};

pub const EPSILON: f64 = 1.0e-7;

pub const BLACK: Color = Color {
    rgb: Tuple {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    },
};
pub const WHITE: Color = Color {
    rgb: Tuple {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    },
};
pub const GREY: Color = Color {
    rgb: Tuple {
        x: 0.5,
        y: 0.5,
        z: 0.5,
    },
};
pub const GREEN: Color = Color {
    rgb: Tuple {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    },
};
pub const BLUE: Color = Color {
    rgb: Tuple {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    },
};
pub const RED: Color = Color {
    rgb: Tuple {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    },
};
