use std::fs::File;

use serde::{Deserialize, Serialize};

fn main() {
    let file_path = "config.yaml";
    let file = File::open(file_path).expect("Unable to open file");
    let res: Config = serde_yaml::from_reader(file).unwrap();
    println!("{:?}", res);
    // let config = Config {
    //     objects: Some(vec![ObjectConfig {
    //         shape: Some(ShapeConfig::Cone(0.7, 0.6, true)),
    //         ..Default::default()
    //     }]),
    //     ..Default::default()
    // };
    // let res = serde_yaml::to_string(&config).unwrap();
    // println!("{}", res)
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
struct Config {
    light: Option<LightConfig>,
    camera: Option<CameraConfig>,
    objects: Option<Vec<ObjectConfig>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
struct LightConfig {
    position: Option<TupleConfig>,
    color: Option<TupleConfig>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
struct TupleConfig(f64, f64, f64);

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
struct CameraConfig {
    from: Option<TupleConfig>,
    to: Option<TupleConfig>,
    up: Option<TupleConfig>,
    width: Option<i32>,
    height: Option<i32>,
    fov_radian: Option<f64>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
struct ObjectConfig {
    shape: Option<ShapeConfig>,
    material: Option<MaterialConfig>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
struct MaterialConfig {
    color: Option<TupleConfig>,
    ambient: Option<f64>,
    diffuse: Option<f64>,
    specular: Option<f64>,
    shininess: Option<f64>,
    pattern: Option<PatternConfig>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum PatternConfig {
    Checker(TupleConfig, TupleConfig),
    Stripe(TupleConfig, TupleConfig),
    Gradient(TupleConfig, TupleConfig),
    Ring(TupleConfig, TupleConfig),
}

impl Default for PatternConfig {
    fn default() -> Self {
        PatternConfig::Checker(TupleConfig::default(), TupleConfig(1.0, 1.0, 1.0))
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
enum ShapeConfig {
    Plane,
    #[default]
    Sphere,
    Cube,
    Cylinder(f64, f64, bool),
    Cone(f64, f64, bool),
}
