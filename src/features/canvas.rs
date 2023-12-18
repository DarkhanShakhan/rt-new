use super::{color::Color, consts::BLACK};

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub canvas: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            canvas: vec![vec![BLACK; width]; height],
        }
    }

    pub fn to_ppm(&self) -> String {
        let mut content = format!("P3\n{} {}\n255\n", self.width, self.height);
        for line in self.canvas.clone().into_iter() {
            for pixel in line {
                content.push_str(&pixel.clamp().rgb.as_str())
            }
        }
        content
    }
}
