use std::{f64::consts::PI, fs::File, io::Write};

use rt_new::features::{
    rotation_x, rotation_y, rotation_z, scaling, translation, view_transformation, Camera, Color,
    Light, Material, Object, Pattern, Point, Vector, World, WHITE,
};

fn main() -> std::io::Result<()> {
    let floor = Object::plane_builder()
        .material(Material {
            pattern: Some(Pattern::checker(WHITE, Color::new(0.5, 0.5, 0.5))),
            ..Default::default()
        })
        .build();
    let left_wall = Object::plane_builder()
        .material(Material {
            pattern: Some(Pattern::checker(WHITE, Color::new(0.5, 0.5, 0.5))),
            ..Default::default()
        })
        .transformation(translation(-15.0, 0.0, 0.0) * rotation_z(PI / 2.0))
        .build();
    let right_wall = Object::plane_builder()
        .material(Material {
            pattern: Some(Pattern::checker(WHITE, Color::new(0.5, 0.5, 0.5))),
            ..Default::default()
        })
        .transformation(translation(0.0, 0.0, 15.0) * rotation_x(PI / 2.0))
        .build();
    let cube = Object::cube_builder()
        .material(Material {
            color: Color::new(0.2, 1.0, 0.3),
            refractive_index: 1.0,
            ..Default::default()
        })
        .transformation(rotation_y(PI / 3.0) * scaling(2.0, 2.0, 2.0) * translation(0.0, 1.0, 0.0))
        .build();
    let cylinder = Object::cylinder_builder(0.0, 4.0, false)
        .material(Material {
            color: Color::new(0.2, 0.4, 1.0),
            refractive_index: 1.0,
            ..Default::default()
        })
        .transformation(translation(-4.0, 1.0, -6.0) * rotation_x(PI / 2.0))
        .build();
    let cylinder_on_cube = Object::cylinder_builder(0.0, 2.0, true)
        .material(Material {
            diffuse: 0.6,
            color: Color::new(0.2, 0.4, 1.0),
            refractive_index: 1.0,
            ..Default::default()
        })
        .transformation(translation(0.0, 3.0, 0.0))
        .build();
    let sphere = Object::sphere_builder()
        .material(Material {
            color: Color::new(0.2, 1.0, 0.3),
            refractive_index: 1.0,
            ..Default::default()
        })
        .transformation(translation(-6.0, 2.0, 2.0) * scaling(2.0, 2.0, 2.0))
        .build();

    let light = Light::new(Point::new(-5.0, 10.0, -10.0), WHITE);
    let mut world = World::new(light);
    world.add_shapes(vec![
        floor,
        left_wall,
        right_wall,
        cube,
        sphere,
        cylinder,
        cylinder_on_cube,
    ]);
    let from = Point::new(7.0, 10.5, -6.5);
    let to = Point::new(0.0, 0.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    let width = 1200;
    let height = 1200;
    let fov = PI / 3.5;
    let mut camera = Camera::new(width as f64, height as f64, fov);
    camera.transform = view_transformation(from, to, up);
    let content = camera.render(&world).to_ppm();
    let mut file = File::create("samples/sample_4.ppm")?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
