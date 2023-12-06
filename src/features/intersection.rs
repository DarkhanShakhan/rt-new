use super::{object::Object, ray::Ray};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Intersection<'a> {
    pub object: &'a Object,
    pub t: f64,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Object) -> Self {
        Intersection { t, object }
    }
    pub fn intersects(object: &'a Object, r: &Ray) -> Option<Vec<Self>> {
        if let Some(intersects) = object.intersect(r) {
            let ixs = intersects
                .iter()
                .map(|t| Intersection::new(*t, object))
                .collect();
            return Some(ixs);
        }
        None
    }
}

pub fn sort_intersections(xs: &mut [Intersection]) {
    xs.sort_by(|a, b| a.t.total_cmp(&b.t));
}

pub fn hit(xs: Vec<Intersection>) -> Option<Intersection> {
    xs.into_iter().find(|i| i.t > 0.0)
}

#[cfg(test)]
mod intersection_tests {
    use super::*;
    #[test]
    fn intersection_encapsulates_t_and_object() {
        let sphere = Object::sphere_builder().build();
        let ix = Intersection::new(3.5, &sphere);
        assert_eq!(ix.t, 3.5);
        assert_eq!(&sphere, ix.object);
    }
}

#[cfg(test)]
mod hit_tests {

    use super::*;
    #[test]
    fn all_intersections_have_positive_t() {
        let sphere = Object::sphere_builder().build();
        let i1 = Intersection::new(1.0, &sphere);
        let i2 = Intersection::new(2.0, &sphere);
        let mut xs = vec![i1, i2];
        sort_intersections(&mut xs);
        let i = hit(xs).unwrap();
        assert_eq!(i.t, 1.0);
    }
    #[test]
    fn some_intersections_have_negative_t() {
        let sphere = Object::sphere_builder().build();
        let i1 = Intersection::new(-1.0, &sphere);
        let i2 = Intersection::new(1.0, &sphere);
        let mut xs = vec![i1, i2];
        sort_intersections(&mut xs);
        let i = hit(xs).unwrap();
        assert_eq!(i.t, 1.0);
    }

    #[test]
    fn when_all_intersections_have_negative_t() {
        let sphere = Object::sphere_builder().build();
        let i1 = Intersection::new(-2.0, &sphere);
        let i2 = Intersection::new(-1.0, &sphere);
        let mut xs = vec![i1, i2];
        sort_intersections(&mut xs);
        assert_eq!(None, hit(xs));
    }

    #[test]
    fn always_the_lowest_nonnegative_intersection() {
        let sphere = Object::sphere_builder().build();
        let i1 = Intersection::new(5.0, &sphere);
        let i2 = Intersection::new(7.0, &sphere);
        let i3 = Intersection::new(-3.0, &sphere);
        let i4 = Intersection::new(2.0, &sphere);
        let mut xs = vec![i1, i2, i3, i4];
        sort_intersections(&mut xs);
        assert_eq!(2.0, hit(xs).unwrap().t);
    }
}

#[cfg(test)]
mod ray_sphere_intersection_tests {
    use crate::features::{
        computation::Computation,
        consts::EPSILON,
        point::Point,
        transformations::{scaling, translation},
        vector::Vector,
    };

    use super::*;
    #[test]
    fn scaled_sphere_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Object::sphere_builder()
            .transformation(scaling(2.0, 2.0, 2.0))
            .build();
        let xs = s.intersect(&r);
        assert_ne!(None, xs);
        let xs = xs.unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 3.0);
        assert_eq!(xs[1], 7.0);
    }

    #[test]
    fn translated_sphere_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Object::sphere_builder()
            .transformation(scaling(5.0, 0.0, 0.0))
            .build();
        let xs = s.intersect(&r);
        assert_eq!(None, xs)
    }
    #[test]
    fn his_offset_point() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Object::sphere_builder()
            .transformation(translation(0.0, 0.0, 1.0))
            .build();
        let i = Intersection::new(5.0, &shape);
        let comps = Computation::new(&r, &i, &[]);
        assert!(comps.over_point.position.z < (-EPSILON / 2.0));
        assert!(comps.point.position.z > comps.over_point.position.z);
    }
}
