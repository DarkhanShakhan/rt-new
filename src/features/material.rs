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
    pub fn builder() -> MaterialBuilder {
        MaterialBuilder::default()
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
        Material {
            color: WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
        }
    }
}

#[derive(Default)]
pub struct MaterialBuilder {
    color: Option<Color>,
    ambient: Option<f64>,
    diffuse: Option<f64>,
    specular: Option<f64>,
    shininess: Option<f64>,
    pattern: Option<Pattern>,
    reflective: Option<f64>,
    transparency: Option<f64>,
    refractive_index: Option<f64>,
}

impl MaterialBuilder {
    pub fn color(mut self, color: Color) -> MaterialBuilder {
        self.color = Some(color);
        self
    }
    pub fn ambient(mut self, ambient: f64) -> MaterialBuilder {
        self.ambient = Some(ambient);
        self
    }
    pub fn diffuse(mut self, diffuse: f64) -> MaterialBuilder {
        self.diffuse = Some(diffuse);
        self
    }
    pub fn specular(mut self, specular: f64) -> MaterialBuilder {
        self.specular = Some(specular);
        self
    }
    pub fn shininess(mut self, shininess: f64) -> MaterialBuilder {
        self.shininess = Some(shininess);
        self
    }
    pub fn pattern(mut self, pattern: Pattern) -> MaterialBuilder {
        self.pattern = Some(pattern);
        self
    }
    pub fn reflective(mut self, reflective: f64) -> MaterialBuilder {
        self.reflective = Some(reflective);
        self
    }
    pub fn transparency(mut self, transparency: f64) -> MaterialBuilder {
        self.transparency = Some(transparency);
        self
    }
    pub fn refractive_index(mut self, refractive_index: f64) -> MaterialBuilder {
        self.refractive_index = Some(refractive_index);
        self
    }
    pub fn build(self) -> Material {
        Material {
            color: self.color.unwrap_or(WHITE),
            ambient: self.ambient.unwrap_or(0.1),
            diffuse: self.diffuse.unwrap_or(0.9),
            specular: self.specular.unwrap_or(0.9),
            shininess: self.shininess.unwrap_or(200.0),
            pattern: self.pattern,
            reflective: self.reflective.unwrap_or_default(),
            transparency: self.transparency.unwrap_or_default(),
            refractive_index: self.refractive_index.unwrap_or(1.0),
        }
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

    use super::*;
    #[test]
    fn transparency_inferective_index() {
        let m = Material::default();
        assert_eq!(m.transparency, 0.0);
        assert_eq!(m.refractive_index, 1.0);
    }
}
