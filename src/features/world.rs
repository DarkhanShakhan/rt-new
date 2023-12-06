use super::{
    color::Color,
    computation::Computation,
    consts::BLACK,
    intersection::{hit, sort_intersections, Intersection},
    light::Light,
    material::Material,
    object::Object,
    point::Point,
    ray::Ray,
    transformations::scaling,
};
pub struct World {
    pub light: Light,
    pub objects: Vec<Object>,
}

impl World {
    pub fn new(light: Light) -> Self {
        World {
            light,
            objects: vec![],
        }
    }
    pub fn add_shapes(&mut self, objects: Vec<Object>) {
        for object in objects {
            self.add_shape(object)
        }
    }
    pub fn add_shape(&mut self, object: Object) {
        self.objects.push(object)
    }
    pub fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let mut result = vec![];
        for object in &self.objects {
            if let Some(ixs) = Intersection::intersects(object, ray) {
                let mut ixs = ixs;
                result.append(&mut ixs);
            }
        }
        if !result.is_empty() {
            sort_intersections(&mut result);
            return Some(result);
        }
        None
    }
    pub fn shade_hit(&self, comps: &Computation, remaining: usize) -> Color {
        let surface = comps.object.material.lighting(
            &self.light,
            comps.object,
            &comps.point,
            &comps.eyev,
            &comps.normalv,
            self.is_shadowed(&comps.over_point),
        );
        let reflected = self.reflected_color(comps, remaining);
        surface + reflected
    }

    pub fn color_at(&self, ray: &Ray, remaining: usize) -> Color {
        if let Some(ixs) = self.intersect(ray) {
            if let Some(hit) = hit(ixs) {
                let comps = Computation::new(ray, &hit, &[]);
                return self.shade_hit(&comps, remaining);
            }
        }
        BLACK
    }
    pub fn is_shadowed(&self, point: &Point) -> bool {
        let v = self.light.position - *point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(*point, direction);
        if let Some(ixs) = self.intersect(&r) {
            if let Some(h) = hit(ixs) {
                if h.t < distance {
                    return true;
                }
            }
        }
        false
    }
    pub fn reflected_color(&self, comps: &Computation, remaining: usize) -> Color {
        if remaining == 0 {
            return BLACK;
        }
        if comps.object.material.reflective == 0.0 {
            return BLACK;
        }
        let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
        let color = self.color_at(&reflect_ray, remaining - 1);
        color * comps.object.material.reflective
    }
}

impl Default for World {
    fn default() -> Self {
        let mut w = World::new(Light::new(
            Point::new(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        let s1 = Object::sphere_builder()
            .material(Material {
                color: Color::new(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ..Default::default()
            })
            .build();
        let s2 = Object::sphere_builder()
            .transformation(scaling(0.5, 0.5, 0.5))
            .build();
        w.add_shape(s1);
        w.add_shape(s2);
        w
    }
}

#[cfg(test)]
mod world_tests {

    use crate::features::{computation::Computation, transformations::translation, vector::Vector};

    use super::*;
    #[test]
    fn intersect_world_with_ray() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = w.intersect(&r).unwrap();
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn shade_intersection() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, &w.objects[0]);
        let comps = Computation::new(&r, &i, &[]);
        let c = w.shade_hit(&comps, 4);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shade_intersection_from_inside() {
        let w = World {
            light: Light::new(Point::new(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0)),
            ..Default::default()
        };
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let i = Intersection::new(0.5, &w.objects[1]);
        let comps = Computation::new(&r, &i, &[]);
        let c = w.shade_hit(&comps, 4);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }
    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let c = w.color_at(&r, 4);
        assert_eq!(c, BLACK);
    }
    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.color_at(&r, 4);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }
    #[test]
    fn color_intersection_behind_ray() {
        let mut w = World::default();
        w.objects[0].material.ambient = 1.0;

        w.objects[1].material.ambient = 1.0;

        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let c = w.color_at(&r, 4);
        assert_eq!(c, w.objects[1].material.color);
    }
    #[test]
    fn no_shadow() {
        let w = World::default();
        let p = Point::new(0.0, 10.0, 0.0);
        assert!(!w.is_shadowed(&p));
    }
    #[test]
    fn shadow_when_object_between_point_and_light() {
        let w = World::default();
        let p = Point::new(10.0, -10.0, 10.0);
        assert!(w.is_shadowed(&p));
    }
    #[test]
    fn no_shadow_when_object_behind_light() {
        let w = World::default();
        let p = Point::new(-20.0, 20.0, -20.0);
        assert!(!w.is_shadowed(&p));
    }
    #[test]
    fn no_shadow_when_object_behind_point() {
        let w = World::default();
        let p = Point::new(-2.0, 2.0, -2.0);
        assert!(!w.is_shadowed(&p));
    }
    #[test]
    fn shade_hit_given_intersection_in_shadow() {
        let mut w = World::new(Light::new(
            Point::new(0.0, 0.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        w.add_shape(Object::sphere_builder().build());
        w.add_shape(
            Object::sphere_builder()
                .transformation(translation(0.0, 0.0, 10.0))
                .build(),
        );
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, &w.objects[1]);
        let comps = Computation::new(&ray, &i, &[]);
        let c = w.shade_hit(&comps, 4);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
    #[test]
    fn shade_hit_reflective_material() {
        let mut w = World::default();
        let shape = Object::plane_builder()
            .material(Material {
                reflective: 0.5,
                ..Default::default()
            })
            .transformation(translation(0.0, -1.0, 0.0))
            .build();
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -(2.0_f64.sqrt() / 2.0), 2.0_f64.sqrt() / 2.0),
        );

        let i = Intersection::new(2.0_f64.sqrt(), &shape);
        let comps = Computation::new(&r, &i, &[]);
        let color = w.shade_hit(&comps, 4);
        w.add_shape(shape);
        assert_eq!(
            color,
            Color::new(0.8767577093610361, 0.9243407894559197, 0.8291746292661524)
        )
    }
    #[test]
    fn reflective_color_at_max_recursive_depth() {
        let mut w = World::default();
        let shape = Object::plane_builder()
            .transformation(translation(0.0, -1.0, 0.0))
            .material(Material {
                reflective: 0.5,
                ..Default::default()
            })
            .build();
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -(2.0_f64.sqrt() / 2.0), 2.0_f64.sqrt() / 2.0),
        );

        let i = Intersection::new(2.0_f64.sqrt(), &shape);
        let comps = Computation::new(&r, &i, &[]);
        let color = w.reflected_color(&comps, 0);
        w.add_shape(shape);
        assert_eq!(color, BLACK)
    }
    #[test]
    fn reflected_color_for_nonreflective_material() {
        let mut w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        w.objects[1].material.ambient = 1.0;
        let i = Intersection::new(1.0, &w.objects[1]);
        let comps = Computation::new(&r, &i, &[]);
        let color = w.reflected_color(&comps, 1);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }
    #[test]
    fn reflected_color_for_reflective_material() {
        let mut w = World::default();
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -(2.0_f64.sqrt() / 2.0), 2.0_f64.sqrt() / 2.0),
        );
        let shape = Object::plane_builder()
            .transformation(translation(0.0, -1.0, 0.0))
            .material(Material {
                reflective: 0.5,
                ..Default::default()
            })
            .build();
        let i = Intersection::new(2.0_f64.sqrt(), &shape);
        let comps = Computation::new(&r, &i, &[]);
        let color = w.reflected_color(&comps, 1);
        w.add_shape(shape);
        assert_eq!(
            color,
            Color::new(
                0.19033232037953468,
                0.23791540047441834,
                0.14274924028465102
            )
        )
    }
    #[test]
    fn color_at_mutually_reflective_surfaces() {
        let mut w = World {
            light: Light::new(Point::new(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0)),
            ..Default::default()
        };
        let lower = Object::plane_builder()
            .transformation(translation(0.0, -1.0, 0.0))
            .material(Material {
                reflective: 1.0,
                ..Default::default()
            })
            .build();
        let upper = Object::plane_builder()
            .transformation(translation(0.0, 1.0, 0.0))
            .material(Material {
                reflective: 1.0,
                ..Default::default()
            })
            .build();
        w.add_shapes(vec![lower, upper]);
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let color = w.color_at(&r, 4);
        assert_eq!(color, Color::new(1.9, 1.9, 1.9))
    }
}
