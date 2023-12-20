use std::fs::File;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Config {
    light: Option<LightConfig>,
    camera: Option<CameraConfig>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct LightConfig {
    position: Option<TupleConfig>,
    color: Option<TupleConfig>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TupleConfig(f64, f64, f64);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct CameraConfig {
    from: Option<TupleConfig>,
    to: Option<TupleConfig>,
    up: Option<TupleConfig>,
}

fn main() {
    let file_path = "config.yaml";
    let file = File::open(file_path).expect("Unable to open file");
    let res: Config = serde_yaml::from_reader(file).unwrap();
    println!("{:?}", res)
}
