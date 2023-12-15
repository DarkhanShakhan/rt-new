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
        if comps.object.material.reflective > 0.0 && comps.object.material.transparency > 0.0 {
            let reflectance = comps.shlick();
            return surface
                + self.reflected_color(comps, remaining) * reflectance
                + self.refracted_color(comps, remaining) * (1.0 - reflectance);
        }
        surface + self.reflected_color(comps, remaining) + self.refracted_color(comps, remaining)
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
    pub fn refracted_color(&self, comps: &Computation, remaining: usize) -> Color {
        if comps.object.material.transparency == 0.0 || remaining == 0 {
            return BLACK;
        }
        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eyev.dot_product(&comps.normalv);
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));
        if sin2_t > 1.0 {
            return BLACK;
        }
        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;
        let refract_ray = Ray::new(comps.under_point, direction);
        self.color_at(&refract_ray, remaining - 1) * comps.object.material.transparency
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

    use crate::features::{
        computation::Computation, transformations::translation, vector::Vector, Pattern,
    };

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
    #[test]
    fn refracted_color_with_opaque_surface() {
        let w = World::default();
        let shape = &w.objects[0];
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut xs = [Intersection::new(4.0, shape), Intersection::new(6.0, shape)];
        sort_intersections(&mut xs);
        let comps = Computation::new(&r, &xs[0], &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, BLACK);
    }

    #[test]
    fn refracted_color_at_maximum_recursive_depth() {
        let mut w = World::default();
        w.objects[0].material.transparency = 1.0;
        w.objects[0].material.refractive_index = 1.5;
        let shape = &w.objects[0];
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut xs = [Intersection::new(4.0, shape), Intersection::new(6.0, shape)];
        sort_intersections(&mut xs);
        let comps = Computation::new(&r, &xs[0], &xs);
        let c = w.refracted_color(&comps, 0);
        assert_eq!(c, BLACK)
    }
    #[test]
    fn refracted_color_under_total_internal_reflection() {
        let mut w = World::default();
        w.objects[0].material.transparency = 1.0;
        w.objects[0].material.refractive_index = 1.5;
        let shape = &w.objects[0];
        let r = Ray::new(
            Point::new(0.0, 0.0, 2.0_f64.sqrt() / 2.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        let mut xs = [
            Intersection::new(-(2.0_f64.sqrt() / 2.0), shape),
            Intersection::new(2.0_f64.sqrt() / 2.0, shape),
        ];
        sort_intersections(&mut xs);
        let comps = Computation::new(&r, &xs[1], &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, BLACK)
    }
    #[test]
    fn refracted_color_with_refracted_ray() {
        let mut w = World::default();
        w.objects[0].material.ambient = 1.0;
        w.objects[0].material.pattern = Some(Pattern::test());
        w.objects[1].material.transparency = 1.0;
        w.objects[1].material.refractive_index = 1.5;
        let a = &w.objects[0];
        let b = &w.objects[1];
        let r = Ray::new(Point::new(0.0, 0.0, 0.1), Vector::new(0.0, 1.0, 0.0));
        let mut xs = [
            Intersection::new(-0.9899, a),
            Intersection::new(-0.4899, b),
            Intersection::new(0.4899, b),
            Intersection::new(0.9899, a),
        ];
        sort_intersections(&mut xs);
        let comps = Computation::new(&r, &xs[2], &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, Color::new(0.0, 0.9988846813665367, 0.04721645191320928));
    }
    #[test]
    fn shade_hit_with_transparent_material() {
        let mut w = World::default();
        let floor = Object::plane_builder()
            .transformation(translation(0.0, -1.0, 0.0))
            .material(Material {
                transparency: 0.5,
                refractive_index: 1.5,
                ..Default::default()
            })
            .build();
        w.add_shape(floor);
        let ball = Object::sphere_builder()
            .transformation(translation(0.0, -3.5, -0.5))
            .material(Material {
                color: Color::new(1.0, 0.0, 0.0),
                ambient: 0.5,
                ..Default::default()
            })
            .build();
        w.add_shape(ball);
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -(2.0_f64.sqrt() / 2.0), 2.0_f64.sqrt() / 2.0),
        );
        let xs = [Intersection::new(2.0_f64.sqrt(), &w.objects[2])];
        let comps = Computation::new(&r, &xs[0], &xs);
        let color = w.shade_hit(&comps, 5);
        assert_eq!(
            color,
            Color::new(0.9364253889815014, 0.6864253889815014, 0.6864253889815014)
        );
    }
    #[test]
    fn shade_hit_with_reflective_transparent_material() {
        let mut w = World::default();
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -(2.0_f64.sqrt() / 2.0), 2.0_f64.sqrt() / 2.0),
        );
        let floor = Object::plane_builder()
            .transformation(translation(0.0, -1.0, 0.0))
            .material(Material {
                reflective: 0.5,
                transparency: 0.5,
                refractive_index: 1.5,
                ..Default::default()
            })
            .build();
        let ball = Object::sphere_builder()
            .transformation(translation(0.0, -3.5, -0.5))
            .material(Material {
                color: Color::new(1.0, 0.0, 0.0),
                ambient: 0.5,
                ..Default::default()
            })
            .build();
        w.add_shapes(vec![floor, ball]);
        let xs = [Intersection::new(2.0_f64.sqrt(), &w.objects[2])];
        let comps = Computation::new(&r, &xs[0], &xs);
        let color = w.shade_hit(&comps, 5);
        assert_eq!(
            color,
            Color::new(0.9339151412754023, 0.696434227200244, 0.692430691912747)
        )
    }
}
