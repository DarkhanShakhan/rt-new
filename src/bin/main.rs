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
    filename: Option<String>,
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
            String::from("samples/") + &self.filename.unwrap_or("example1".to_string()) + ".ppm",
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
        builder.build()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
enum PatternConfig {
    Checker(TupleConfig, TupleConfig),
    Stripe(TupleConfig, TupleConfig),
    Gradient(TupleConfig, TupleConfig),
    Ring(TupleConfig, TupleConfig),
}

impl From<PatternConfig> for Pattern {
    fn from(value: PatternConfig) -> Self {
        match value {
            PatternConfig::Checker(color_a, color_b) => {
                Pattern::checker(Color::from(color_a), Color::from(color_b))
            }
            PatternConfig::Stripe(color_a, color_b) => {
                Pattern::stripe(Color::from(color_a), Color::from(color_b))
            }
            PatternConfig::Gradient(color_a, color_b) => {
                Pattern::gradient(Color::from(color_a), Color::from(color_b))
            }
            PatternConfig::Ring(color_a, color_b) => {
                Pattern::ring(Color::from(color_a), Color::from(color_b))
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
enum ShapeConfig {
    Plane,
    #[default]
    Sphere,
    Cube,
    Cylinder(f64, f64, bool),
    Cone(f64, f64, bool),
}

impl From<ShapeConfig> for Shape {
    fn from(value: ShapeConfig) -> Self {
        match value {
            ShapeConfig::Plane => Shape::Plane,
            ShapeConfig::Sphere => Shape::Sphere,
            ShapeConfig::Cube => Shape::Cube,
            ShapeConfig::Cylinder(min, max, closed) => Shape::Cylinder(min, max, closed),
            ShapeConfig::Cone(min, max, closed) => Shape::Cone(min, max, closed),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
enum TransformationConfig {
    Translation(f64, f64, f64),
    Scaling(f64, f64, f64),
    RotationX(f64),
    RotationY(f64),
    RotationZ(f64),
}

impl From<TransformationConfig> for Matrice {
    fn from(value: TransformationConfig) -> Self {
        match value {
            TransformationConfig::Translation(x, y, z) => translation(x, y, z),
            TransformationConfig::Scaling(x, y, z) => scaling(x, y, z),
            TransformationConfig::RotationX(rad) => rotation_x(rad),
            TransformationConfig::RotationY(rad) => rotation_y(rad),
            TransformationConfig::RotationZ(rad) => rotation_z(rad),
        }
    }
}
//TODO:
//object transform
//file name
//defaults
