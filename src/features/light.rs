use super::{color::Color, point::Point, WHITE};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Light {
    pub position: Point,
    pub intensity: Color,
}

impl Light {
    pub fn new(position: Point, intensity: Color) -> Self {
        Light {
            position,
            intensity,
        }
    }
}
impl Default for Light {
    fn default() -> Self {
        Light::new(Point::new(-5.0, 10.0, -10.0), WHITE)
    }
}

#[cfg(test)]
mod light_tests {

    use super::*;

    #[test]
    fn test_creating_light() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Point::new(0.0, 0.0, 0.0);
        let light = Light::new(position, intensity);
        assert_eq!(light.intensity, intensity);
        assert_eq!(light.position, position);
    }
}
