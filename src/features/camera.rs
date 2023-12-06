use super::{canvas::Canvas, matrice::Matrice, point::Point, ray::Ray, world::World};
use indicatif::ProgressBar;
extern crate rayon;
use rayon::prelude::*;

pub struct Camera {
    pub hsize: f64,
    pub vsize: f64,
    pub field_of_view: f64,
    pub transform: Matrice,
    pub pixel_size: f64,
    pub half_width: f64,
    pub half_height: f64,
}

const REMAINING: usize = 4;
impl Camera {
    pub fn new(hsize: f64, vsize: f64, field_of_view: f64) -> Self {
        let half_width: f64;
        let half_height: f64;
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize / vsize;
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrice::identity(),
            pixel_size: (half_width * 2.0) / hsize,
            half_width,
            half_height,
        }
    }
    pub fn ray_for_pixel(&self, px: f64, py: f64) -> Ray {
        let xoffset = (px + 0.5) * self.pixel_size;
        let yoffset = (py + 0.5) * self.pixel_size;
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;
        let pixel = &self.transform.inverse() * &Point::new(world_x, world_y, -1.0);
        let origin = &self.transform.inverse() * &Point::new(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }
    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize as usize, self.vsize as usize);
        let bar = ProgressBar::new((self.vsize * self.hsize) as u64);
        image
            .canvas
            .par_iter_mut()
            .enumerate()
            .for_each(|(y, row)| {
                row.par_iter_mut().enumerate().for_each(|(x, pixel)| {
                    let ray = self.ray_for_pixel(x as f64, y as f64);
                    bar.inc(1);
                    *pixel = world.color_at(&ray, REMAINING);
                })
            });
        image
    }
}

#[cfg(test)]
mod camera_tests {
    use std::f64::consts::PI;

    use crate::features::matrice::Matrice;

    use super::Camera;

    #[test]
    fn construct_camera() {
        let c = Camera::new(160.0, 120.0, PI / 2.0);
        assert_eq!(c.hsize, 160.0);
        assert_eq!(c.vsize, 120.0);
        assert_eq!(c.field_of_view, PI / 2.0);
        assert_eq!(c.transform, Matrice::identity());
    }
    #[test]
    fn pixel_size_horizontal_canvas() {
        let c = Camera::new(200.0, 125.0, PI / 2.0);
        assert_eq!(c.pixel_size, 0.009999999999999998)
    }
    #[test]
    fn pixel_size_vertical_canvas() {
        let c = Camera::new(125.0, 200.0, PI / 2.0);
        assert_eq!(c.pixel_size, 0.009999999999999998)
    }
}

#[cfg(test)]
mod ray_for_pixel_tests {
    use std::f64::consts::PI;

    use crate::features::{
        transformations::{rotation_y, translation},
        vector::Vector,
    };

    use super::*;
    #[test]
    fn test_ray_through_canvas_center() {
        let c = Camera::new(201.0, 101.0, PI / 2.0);
        let r = c.ray_for_pixel(100.0, 50.0);
        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_ray_through_canvas_corner() {
        let c = Camera::new(201.0, 101.0, PI / 2.0);
        let r = c.ray_for_pixel(0.0, 0.0);
        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn test_ray_camera_transformed() {
        let mut c = Camera::new(201.0, 101.0, PI / 2.0);
        c.transform = rotation_y(PI / 4.0) * translation(0.0, -2.0, 5.0);
        let r = c.ray_for_pixel(100.0, 50.0);
        assert_eq!(r.origin, Point::new(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            Vector::new(2.0_f64.sqrt() / 2.0, 0.0, -(2.0_f64.sqrt() / 2.0))
        );
    }
}

#[cfg(test)]
mod render_tests {
    use std::f64::consts::PI;

    use crate::features::{color::Color, transformations::view_transformation, vector::Vector};

    use super::*;
    #[test]
    fn test_render_world_with_camera() {
        let world = World::default();
        let mut camera = Camera::new(11.0, 11.0, PI / 2.0);
        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        camera.transform = view_transformation(from, to, up);
        let image = camera.render(&world);
        assert_eq!(image.canvas[5][5], Color::new(0.38066, 0.47583, 0.2855));
    }
}
