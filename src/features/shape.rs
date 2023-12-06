use super::{consts::EPSILON, point::Point, ray::Ray, vector::Vector};

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub enum Shape {
    Plane,
    #[default]
    Sphere,
}

impl Shape {
    pub fn intersect(&self, ray: &Ray) -> Option<Vec<f64>> {
        match self {
            Shape::Plane => intersect_plane(ray),
            Shape::Sphere => intersect_sphere(ray),
        }
    }
    pub fn normal_at(&self, object_point: &Point) -> Vector {
        match self {
            Shape::Plane => normal_at_plane(),
            Shape::Sphere => normal_at_sphere(object_point),
        }
    }
}

// PLANE
fn normal_at_plane() -> Vector {
    Vector::new(0.0, 1.0, 0.0)
}

fn intersect_plane(ray: &Ray) -> Option<Vec<f64>> {
    if ray.direction.position.y.abs() < EPSILON {
        return None;
    }
    Some(vec![-ray.origin.position.y / ray.direction.position.y])
}

//SHAPE
fn normal_at_sphere(object_point: &Point) -> Vector {
    *object_point - Point::new(0.0, 0.0, 0.0)
}

fn intersect_sphere(ray: &Ray) -> Option<Vec<f64>> {
    let sphere_to_ray = ray.origin - Point::default();
    let a = ray.direction.dot_product(&ray.direction);
    let b = 2.0 * ray.direction.dot_product(&sphere_to_ray);
    let c = sphere_to_ray.dot_product(&sphere_to_ray) - 1.0;
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }
    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
    Some(vec![t1, t2])
}
