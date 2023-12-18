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
            color: Color::new(0.2, 0.9, 0.4),
            diffuse: 0.8,
            specular: 0.7,
            refractive_index: 1.0,
            ..Default::default()
        })
        .transformation(rotation_y(PI / 3.0) * scaling(2.0, 2.0, 2.0) * translation(0.0, 1.0, 0.0))
        .build();

    let light = Light::new(Point::new(-5.0, 10.0, -10.0), Color::new(0.8, 0.8, 0.8));
    let mut world = World::new(light);
    world.add_shapes(vec![floor, left_wall, right_wall, cube]);
    let from = Point::new(3.0, 8.5, -14.5);
    let to = Point::new(0.0, 0.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    let width = 1200;
    let height = 1200;
    let fov = PI / 3.5;
    let mut camera = Camera::new(width as f64, height as f64, fov);
    camera.transform = view_transformation(from, to, up);
    let content = camera.render(&world).to_ppm();
    let mut file = File::create("samples/sample_2.ppm")?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
