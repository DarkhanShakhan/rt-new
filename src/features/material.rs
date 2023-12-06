use super::{
    color::Color,
    consts::{BLACK, WHITE},
    light::Light,
    object::Object,
    pattern::Pattern,
    point::Point,
    vector::Vector,
};

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Option<Pattern>,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
}

impl Material {
    pub fn new(
        color: Color,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
        pattern: Option<Pattern>,
        reflective: f64,
        transparency: f64,
        refractive_index: f64,
    ) -> Self {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
            pattern,
            reflective,
            transparency,
            refractive_index,
        }
    }
    pub fn lighting(
        &self,
        light: &Light,
        object: &Object,
        point: &Point,
        eyev: &Vector,
        normalv: &Vector,
        in_shadow: bool,
    ) -> Color {
        let color = match &self.pattern {
            Some(p) => p.at(object, point),
            None => self.color,
        };

        let effective_color = color * light.intensity;
        let ambient = effective_color * self.ambient;
        if in_shadow {
            return ambient;
        }
        let lightv = (light.position - *point).normalize();
        let light_dot_normal = lightv.dot_product(normalv);
        let diffuse: Color;
        let specular: Color;
        if light_dot_normal < 0.0 {
            diffuse = BLACK;
            specular = BLACK;
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = (-lightv).reflect(normalv);
            let reflectv_dot_eye = reflectv.dot_product(eyev);
            if reflectv_dot_eye <= 0.0 {
                specular = BLACK;
            } else {
                let factor = reflectv_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::new(WHITE, 0.1, 0.9, 0.9, 200.0, None, 0.0, 0.0, 1.0)
    }
}

#[cfg(test)]
mod material_tests {
    use super::*;

    #[test]
    fn default_material() {
        let m = Material::default();
        assert_eq!(m.color, WHITE);
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }
}

#[cfg(test)]
mod lighting_tests {

    use super::*;

    #[test]
    fn eye_between_light_and_surface() {
        let m = Material::default();
        let position = Point::default();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(
            &light,
            &Object::default(),
            &position,
            &eyev,
            &normalv,
            false,
        );
        assert_eq!(result, Color::new(1.9, 1.9, 1.9))
    }

    #[test]
    fn eye_between_light_and_surface_offset_45() {
        let m = Material::default();
        let position = Point::default();
        let eyev = Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -(2.0_f64.sqrt() / 2.0));
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(
            &light,
            &Object::default(),
            &position,
            &eyev,
            &normalv,
            false,
        );
        assert_eq!(result, Color::new(1.0, 1.0, 1.0))
    }
    #[test]
    fn eye_opposite_surface_offset_45() {
        let m = Material::default();
        let position = Point::default();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(
            &light,
            &Object::default(),
            &position,
            &eyev,
            &normalv,
            false,
        );
        assert_eq!(
            result,
            Color::new(0.7363961030678927, 0.7363961030678927, 0.7363961030678927)
        )
    }
    #[test]
    fn eye_in_path_of_reflection_vector() {
        let m = Material::default();
        let position = Point::default();
        let eyev = Vector::new(0.0, -(2.0_f64.sqrt() / 2.0), -(2.0_f64.sqrt() / 2.0));
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(
            &light,
            &Object::default(),
            &position,
            &eyev,
            &normalv,
            false,
        );
        assert_eq!(
            result,
            Color::new(1.6363961030678928, 1.6363961030678928, 1.6363961030678928)
        )
    }
    #[test]
    fn light_behind_surface() {
        let m = Material::default();
        let position = Point::default();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(
            &light,
            &Object::default(),
            &position,
            &eyev,
            &normalv,
            false,
        );
        assert_eq!(result, Color::new(0.1, 0.1, 0.1))
    }
    #[test]
    fn with_surface_in_shadow() {
        let m = Material::default();
        let position = Point::default();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = true;
        let result = m.lighting(
            &light,
            &Object::default(),
            &position,
            &eyev,
            &normalv,
            in_shadow,
        );
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}

#[cfg(test)]
mod refraction_tests {
    use crate::features::{scaling, translation, Computation, Intersection, Matrice, Ray};

    use super::*;
    #[test]
    fn transparency_inferective_index() {
        let m = Material::default();
        assert_eq!(m.transparency, 0.0);
        assert_eq!(m.refractive_index, 1.0);
    }

    #[test]
    fn glass_material() {
        let s = glass_sphere();
        assert_eq!(*s.transformation(), Matrice::identity());
        assert_eq!(s.material.transparency, 1.0);
        assert_eq!(s.material.refractive_index, 1.5);
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

    fn glass_sphere() -> Object {
        let m = Material {
            transparency: 1.0,
            refractive_index: 1.5,
            ..Default::default()
        };
        Object::sphere_builder().material(m).build()
    }
}
