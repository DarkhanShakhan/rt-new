use super::{
    consts::EPSILON, intersection::Intersection, object::Object, point::Point, ray::Ray,
    vector::Vector,
};

pub struct Computation<'a> {
    pub t: f64,
    pub object: &'a Object,
    pub point: Point,
    pub over_point: Point,
    pub under_point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
    pub reflectv: Vector,
    pub n1: f64,
    pub n2: f64,
}

impl<'a> Computation<'a> {
    pub fn new(ray: &Ray, i: &Intersection<'a>, xs: &[Intersection<'a>]) -> Self {
        let mut n1: f64 = 1.0;
        let mut n2: f64 = 1.0;
        let mut containers: Vec<&Object> = vec![];
        for x in xs.iter() {
            if *i == *x {
                if let Some(l) = containers.last() {
                    n1 = l.material.refractive_index;
                }
            }
            if let Some(index) = containers.iter().position(|a| *a == x.object) {
                containers.remove(index);
            } else {
                containers.push(x.object)
            }
            if *i == *x {
                if let Some(l) = containers.last() {
                    n2 = l.material.refractive_index;
                }
                break;
            }
        }
        let point = ray.position(i.t);
        let eyev = -ray.direction;
        let mut normalv = i.object.normal_at(&point);
        let inside: bool;
        if normalv.dot_product(&eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        } else {
            inside = false;
        }
        Computation {
            t: i.t,
            object: i.object,
            point,
            over_point: point + normalv * EPSILON,
            under_point: point - normalv * EPSILON,
            eyev,
            normalv,
            inside,
            reflectv: ray.direction.reflect(&normalv),
            n1,
            n2,
        }
    }
    pub fn shlick(&self) -> f64 {
        let mut cos = self.eyev.dot_product(&self.normalv);
        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
            if sin2_t > 1.0 {
                return 1.0;
            }
            cos = (1.0 - sin2_t).sqrt();
        }
        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

#[cfg(test)]
mod computation_tests {

    use crate::features::Material;

    use super::*;
    #[test]
    fn precompute_state_of_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Object::sphere_builder().build();
        let i = Intersection::new(4.0, &shape);
        let comps = Computation::new(&r, &i, &[]);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn intersection_outside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Object::sphere_builder().build();
        let i = Intersection::new(1.0, &shape);
        let comps = Computation::new(&r, &i, &[]);
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn intersection_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Object::sphere_builder().build();
        let i = Intersection::new(1.0, &shape);
        let comps = Computation::new(&r, &i, &[]);
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
    }
    #[test]
    fn precompute_reflection_vector() {
        let shape = Object::plane_builder().build();
        let r = Ray::new(
            Point::new(0.0, 1.0, -1.0),
            Vector::new(0.0, -(2.0_f64.sqrt() / 2.0), 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), &shape);
        let comps = Computation::new(&r, &i, &[]);
        assert_eq!(
            comps.reflectv,
            Vector::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        )
    }
    #[test]
    fn schlick_approximation_under_total_internal_reflection() {
        let shape = glass_sphere();
        let r = Ray::new(
            Point::new(0.0, 0.0, 2.0_f64.sqrt() / 2.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        let xs = vec![
            Intersection::new(-(2.0_f64.sqrt()), &shape),
            Intersection::new(2.0_f64.sqrt() / 2.0, &shape),
        ];
        let comps = Computation::new(&r, &xs[1], &xs);
        let reflectance = comps.shlick();
        assert_eq!(reflectance, 1.0);
    }

    #[test]
    fn schlick_approximation_with_perpendicular_viewing_angle() {
        let shape = glass_sphere();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let xs = [
            Intersection::new(-1.0, &shape),
            Intersection::new(1.0, &shape),
        ];
        let comps = Computation::new(&r, &xs[1], &xs);
        let reflectance = comps.shlick();
        assert_eq!(reflectance, 0.04000000000000001)
    }

    #[test]
    fn schlick_approximation_with_small_angle_and_n2_smaller_n1() {
        let shape = glass_sphere();
        let r = Ray::new(Point::new(0.0, 0.99, -2.0), Vector::new(0.0, 0.0, 1.0));
        let xs = [Intersection::new(1.8589, &shape)];
        let comps = Computation::new(&r, &xs[0], &xs);
        let reflectance = comps.shlick();
        assert_eq!(reflectance, 0.4887308101221217);
    }

    fn glass_sphere() -> Object {
        let m = Material {
            transparency: 1.0,
            refractive_index: 1.5,
            ..Default::default()
        };
        Object::sphere_builder().material(m).build()
    }
}
