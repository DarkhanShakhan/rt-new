use std::f64::consts::PI;

use rt_new::features::{
    rotation_x, rotation_y, rotation_z, scaling, translation, view_transformation, Camera, Color,
    Light, Material, Object, Pattern, Point, Vector, World, WHITE,
};

fn main() {
    let floor = Object::plane_builder()
        .material(Material {
            pattern: Some(Pattern::checker(WHITE, Color::new(0.5, 0.5, 0.5))),
            reflective: 0.0,
            ..Default::default()
        })
        .transformation(translation(0.0, -1.0, 0.0))
        .build();

    let wall_left = Object::plane_builder()
        .material(Material {
            pattern: Some(Pattern::checker(WHITE, Color::new(0.5, 0.5, 0.5))),
            reflective: 0.0,
            ..Default::default()
        })
        .transformation(translation(-15.0, 0.0, 0.0) * rotation_z(PI / 2.0))
        .build();
    let wall_right = Object::plane_builder()
        .material(Material {
            pattern: Some(Pattern::checker(WHITE, Color::new(0.5, 0.5, 0.5))),
            reflective: 0.0,
            ..Default::default()
        })
        .transformation(translation(0.0, 0.0, 15.0) * rotation_x(PI / 2.0))
        .build();
    let glass_sphere_s = Object::sphere_builder()
        .material(Material {
            transparency: 0.8,
            refractive_index: 1.0,
            ..Default::default()
        })
        .transformation(translation(1.0, 1.0, -2.0))
        .build();
    let cylinder = Object::cylinder_builder(0.0, 1.0, true)
        .transformation(scaling(0.25, 1.0, 0.25))
        .build();
    let cone = Object::cone_builder(-1.0, 1.0, false)
        .material(Material {
            color: Color::new(0.4, 0.7, 0.9),
            ..Default::default()
        })
        .transformation(rotation_x(PI / 1.5))
        .build();
    let cone_2 = Object::cone_builder(0.0, 1.0, true)
        // .transformation(translation(0.0, 5.0, 0.0) * scaling(1.0, 2.0, 1.0))
        .build();
    let glass_sphere = Object::cube_builder()
        .material(Material {
            color: WHITE,
            ..Default::default()
        })
        .transformation(translation(1.0, 1.0, -2.0) * rotation_x(PI / 3.0))
        .build();
    let light = Light::new(Point::new(-5.0, 10.0, -10.0), WHITE);
    let mut world = World::new(light);
    world.add_shapes(vec![
        wall_left, floor, cone,
        wall_right,
        // glass_sphere,
        // glass_sphere_s,
        // cone_2,
    ]);
    let from = Point::new(8.0, 2.5, -10.5);
    let to = Point::new(1.5, 3.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    let width = 800;
    let height = 800;
    let fov = PI / 3.5;
    let mut camera = Camera::new(width as f64, height as f64, fov);
    camera.transform = view_transformation(from, to, up);
    camera.render(&world).to_ppm();
}
