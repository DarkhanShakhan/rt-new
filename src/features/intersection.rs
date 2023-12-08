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
        Material, Matrice,
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
    #[test]
    fn find_n1_n2_various_intersections() {
        let mut a = glass_sphere();
        a.set_transformation(scaling(2.0, 2.0, 2.0));
        a.material.refractive_index = 1.5;
        let mut b = glass_sphere();
        b.set_transformation(translation(0.0, 0.0, -0.25));
        b.material.refractive_index = 2.0;
        let mut c = glass_sphere();
        c.set_transformation(translation(0.0, 0.0, 0.25));
        c.material.refractive_index = 2.5;
        let r = Ray::new(Point::new(0.0, 0.0, -4.0), Vector::new(0.0, 0.0, 1.0));
        let xs = [
            Intersection::new(2.0, &a),
            Intersection::new(2.75, &b),
            Intersection::new(3.25, &c),
            Intersection::new(4.75, &b),
            Intersection::new(5.25, &c),
            Intersection::new(6.0, &a),
        ];
        let n12 = [
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];
        for (i, ix) in xs.iter().enumerate() {
            let comps = Computation::new(&r, ix, &xs);
            assert_eq!(comps.n1, n12[i].0);
            assert_eq!(comps.n2, n12[i].1);
        }
    }

    #[test]
    fn under_point_below_surface() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut shape = glass_sphere();
        shape.set_transformation(translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, &shape);
        let xs = [i];
        let comps = Computation::new(&r, &xs[0], &xs);
        assert!(comps.under_point.z() > EPSILON / 2.0);
        assert!(comps.point.z() < comps.under_point.z());
    }
    #[test]
    fn glass_material() {
        let s = glass_sphere();
        assert_eq!(*s.transformation(), Matrice::identity());
        assert_eq!(s.material.transparency, 1.0);
        assert_eq!(s.material.refractive_index, 1.5);
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
