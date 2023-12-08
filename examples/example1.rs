use std::f64::consts::PI;

use rt_new::features::{
    rotation_x, rotation_z, translation, view_transformation, Camera, Color, Light, Material,
    Object, Pattern, Point, Vector, World, WHITE,
};

fn main() {
    let floor = Object::plane_builder()
        .material(Material {
            pattern: Some(Pattern::checker(WHITE, Color::new(0.5, 0.5, 0.5))),
            reflective: 0.0,
            ..Default::default()
        })
        .build();

    let wall_left = Object::plane_builder()
        .material(Material {
            pattern: Some(Pattern::checker(WHITE, Color::new(0.5, 0.5, 0.5))),
            reflective: 0.0,
            ..Default::default()
        })
        .transformation(rotation_z(PI / 2.0) * translation(-15.0, 0.0, 0.0))
        .build();
    let wall_right = Object::plane_builder()
        .material(Material {
            pattern: Some(Pattern::checker(WHITE, Color::new(0.5, 0.5, 0.5))),
            reflective: 0.0,
            ..Default::default()
        })
        .transformation(rotation_x(PI / 2.0) * translation(0.0, 0.0, 15.0))
        .build();
    let glass_sphere_s = Object::sphere_builder()
        .material(Material {
            transparency: 0.8,
            refractive_index: 1.0,
            ..Default::default()
        })
        .transformation(translation(1.0, 1.0, -2.0))
        .build();
    let glass_sphere = Object::sphere_builder()
        .material(Material {
            color: Color::new(0.1, 0.1, 0.1),
            diffuse: 0.3,
            specular: 0.2,
            reflective: 0.00,
            transparency: 1.0,
            refractive_index: 1.5,
            ..Default::default()
        })
        .transformation(translation(3.0, 1.0, -2.0))
        .build();
    let light = Light::new(Point::new(5.0, 10.0, -10.0), WHITE);
    let mut world = World::new(light);
    world.add_shapes(vec![
        wall_left,
        floor,
        wall_right,
        glass_sphere,
        glass_sphere_s,
    ]);
    let from = Point::new(8.0, 2.5, -10.5);
    let to = Point::new(1.5, 3.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    let width = 600;
    let height = 600;
    let fov = PI / 3.5;
    let mut camera = Camera::new(width as f64, height as f64, fov);
    camera.transform = view_transformation(from, to, up);
    camera.render(&world).to_ppm();
}
