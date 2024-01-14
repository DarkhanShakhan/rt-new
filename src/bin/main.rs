use std::{f64::consts::PI, fs::File, io::Write};

use rt_new::features::{
    rotation_x, rotation_y, rotation_z, scaling, translation, view_transformation, Camera, Color,
    Light, Material, Matrice, Object, Pattern, Point, Shape, Vector, World, WHITE,
};
use serde::{Deserialize, Serialize};

fn main() {
    let file_path = "config.yaml";
    let file = File::open(file_path).expect("Unable to open file");
    let res: Config = serde_yaml::from_reader(file).unwrap();
    println!("{:?}", res);
    res.ray_tracer();
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
struct Config {
    light: Option<LightConfig>,
    camera: Option<CameraConfig>,
    objects: Option<Vec<ObjectConfig>>,
    output_file: Option<String>,
}

impl Config {
    pub fn ray_tracer(self) {
        let light = self.light.map_or(Light::default(), Light::from);
        let camera = self.camera.map_or(Camera::default(), Camera::from);
        let objects: Option<Vec<Object>> = self
            .objects
            .map(|objs| objs.iter().map(|o| Object::from(o.clone())).collect());
        let world = objects.map_or(World::new(light.clone()), |objs| {
            let mut w = World::new(light);
            w.add_shapes(objs);
            w
        });
        File::create(
            String::from("samples/") + &self.output_file.unwrap_or("example1".to_string()) + ".ppm",
        )
        .unwrap()
        .write_all(camera.render(&world).to_ppm().as_bytes())
        .unwrap();
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
struct LightConfig {
    position: Option<TupleConfig>,
    color: Option<TupleConfig>,
}

impl From<LightConfig> for Light {
    fn from(value: LightConfig) -> Self {
        Light::new(
            value.position.map_or(Point::default(), Point::from),
            value.color.map_or(WHITE, Color::from),
        )
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
struct TupleConfig(f64, f64, f64);

impl From<TupleConfig> for Point {
    fn from(value: TupleConfig) -> Self {
        Point::new(value.0, value.1, value.2)
    }
}

impl From<TupleConfig> for Vector {
    fn from(value: TupleConfig) -> Self {
        Vector::new(value.0, value.1, value.2)
    }
}

impl From<TupleConfig> for Color {
    fn from(value: TupleConfig) -> Self {
        Color::new(value.0, value.1, value.2)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
struct CameraConfig {
    from: Option<TupleConfig>,
    to: Option<TupleConfig>,
    up: Option<TupleConfig>,
    width: Option<i32>,
    height: Option<i32>,
    fov_radian: Option<f64>,
}

impl From<CameraConfig> for Camera {
    fn from(value: CameraConfig) -> Self {
        let mut res = Camera::new(
            value.width.unwrap_or(1200) as f64,
            value.height.unwrap_or(1200) as f64,
            value.fov_radian.unwrap_or(PI / 3.5),
        );
        res.transform = view_transformation(
            value.from.map_or(Point::new(3.0, 8.5, -14.5), Point::from),
            value.to.map_or(Point::new(0.0, 0.0, 0.0), Point::from),
            value.up.map_or(Vector::new(0.0, 1.0, 0.0), Vector::from),
        );
        res
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
struct ObjectConfig {
    shape: Option<ShapeConfig>,
    material: Option<MaterialConfig>,
    transformation: Option<Vec<TransformationConfig>>,
}

impl ObjectConfig {
    pub fn as_object(&self) -> Object {
        Object::default()
    }
}

impl From<ObjectConfig> for Object {
    fn from(value: ObjectConfig) -> Self {
        Object::new(
            value.material.map_or(Material::default(), Material::from),
            value.shape.map_or(Shape::Sphere, Shape::from),
            value.transformation.map_or(Matrice::default(), |list| {
                list.iter()
                    .fold(Matrice::default(), |acc, x| acc * Matrice::from(x.clone()))
            }),
        )
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
struct MaterialConfig {
    color: Option<TupleConfig>,
    ambient: Option<f64>,
    diffuse: Option<f64>,
    specular: Option<f64>,
    shininess: Option<f64>,
    reflective: Option<f64>,
    transparency: Option<f64>,
    refractive_index: Option<f64>,
    pattern: Option<PatternConfig>,
}

impl From<MaterialConfig> for Material {
    fn from(value: MaterialConfig) -> Self {
        let mut builder = Material::builder();
        if let Some(ambient) = value.ambient {
            builder = builder.ambient(ambient);
        }
        if let Some(diffuse) = value.diffuse {
            builder = builder.diffuse(diffuse);
        }
        if let Some(specular) = value.specular {
            builder = builder.specular(specular);
        }
        if let Some(shininess) = value.shininess {
            builder = builder.shininess(shininess);
        }
        if let Some(color) = value.color {
            builder = builder.color(Color::from(color));
        }
        if let Some(pattern) = value.pattern {
            builder = builder.pattern(Pattern::from(pattern))
        }
        if let Some(reflective) = value.reflective {
            builder = builder.reflective(reflective);
        }
        if let Some(transparency) = value.transparency {
            builder = builder.transparency(transparency);
        }
        if let Some(refractive_index) = value.refractive_index {
            builder = builder.refractive_index(refractive_index);
        }
        builder.build()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct PatternConfig {
    pattern_type: Option<String>,
    color_a: Option<TupleConfig>,
    color_b: Option<TupleConfig>,
}
//     Checker(TupleConfig, TupleConfig),
//     Stripe(TupleConfig, TupleConfig),
//     Gradient(TupleConfig, TupleConfig),
//     Ring(TupleConfig, TupleConfig),
// }

impl From<PatternConfig> for Pattern {
    fn from(value: PatternConfig) -> Self {
        match value.pattern_type.unwrap().as_str() {
            "checker" => Pattern::checker(
                Color::from(value.color_a.unwrap()),
                Color::from(value.color_b.unwrap()),
            ),
            "stripe" => Pattern::stripe(
                Color::from(value.color_a.unwrap()),
                Color::from(value.color_b.unwrap()),
            ),
            "gradient" => Pattern::gradient(
                Color::from(value.color_a.unwrap()),
                Color::from(value.color_b.unwrap()),
            ),
            "ring" => Pattern::ring(
                Color::from(value.color_a.unwrap()),
                Color::from(value.color_b.unwrap()),
            ),
            _ => Pattern::default(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
struct ShapeConfig {
    shape_type: Option<String>,
    min: Option<f64>,
    max: Option<f64>,
    closed: Option<bool>,
}

impl From<ShapeConfig> for Shape {
    fn from(value: ShapeConfig) -> Self {
        match value.shape_type.unwrap().as_str() {
            "sphere" => Shape::Sphere,
            "cone" => Shape::Cone(
                value.min.unwrap(),
                value.max.unwrap(),
                value.closed.unwrap(),
            ),
            "cube" => Shape::Cube,
            "cylinder" => Shape::Cylinder(
                value.min.unwrap(),
                value.max.unwrap(),
                value.closed.unwrap(),
            ),
            "plane" => Shape::Plane,
            _ => Shape::default(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct TransformationConfig {
    transformation_type: Option<String>,
    vec: Option<TupleConfig>,
    rad: Option<f64>,
}

impl From<TransformationConfig> for Matrice {
    fn from(value: TransformationConfig) -> Self {
        match value.transformation_type.unwrap().as_str() {
            "translation" => {
                let tuple = value.vec.unwrap();
                translation(tuple.0, tuple.1, tuple.2)
            }
            "scaling" => {
                let tuple = value.vec.unwrap();
                scaling(tuple.0, tuple.1, tuple.2)
            }
            "rotation_x" => rotation_x(value.rad.unwrap()),
            "rotation_y" => rotation_y(value.rad.unwrap()),
            "rotation_z" => rotation_z(value.rad.unwrap()),
            _ => Matrice::default(),
        }
    }
}
//TODO:
//object transform
//file name
//defaults
