use std::{f64::consts::PI, fs::File, io::Write};

use rt_new::features::{
    rotation_x, rotation_z, translation, view_transformation, Camera, Color, Light, Material,
    Object, Pattern, Point, Vector, World, BLACK, GREY, WHITE,
};

fn main() -> std::io::Result<()> {
    let floor = Object::plane_builder()
        .material(
            Material::builder()
                .pattern(Pattern::checker(WHITE, GREY))
                .build(),
        )
        .transformation(translation(0.0, -10.0, 0.0))
        .build();
    let left_wall = Object::plane_builder()
        .material(
            Material::builder()
                .pattern(Pattern::checker(WHITE, GREY))
                .build(),
        )
        .transformation(translation(-15.0, 0.0, 0.0) * rotation_z(PI / 2.0))
        .build();
    let right_wall = Object::plane_builder()
        .material(
            Material::builder()
                .pattern(Pattern::checker(WHITE, GREY))
                .build(),
        )
        .transformation(translation(0.0, 0.0, 15.0) * rotation_x(PI / 2.0))
        .build();
    let ceiling = Object::plane_builder()
        .material(
            Material::builder()
                .pattern(Pattern::checker(WHITE, Color::new(0.5, 0.5, 0.5)))
                .build(),
        )
        .transformation(translation(0.0, 15.0, 0.0))
        .build();
    let cube = Object::cube_builder()
        .material(
            Material::builder()
                .pattern(Pattern::checker(BLACK, WHITE))
                .build(),
        )
        .transformation(translation(0.0, 1.0, 0.0))
        .build();
    let sphere = Object::sphere_builder()
        .material(
            Material::builder()
                .pattern(Pattern::checker(BLACK, WHITE))
                .build(),
        )
        .transformation(translation(4.0, 1.0, 0.0))
        .build();
    let light = Light::new(Point::new(-5.0, 10.0, -10.0), WHITE);
    let mut world = World::new(light);
    world.add_shapes(vec![floor, left_wall, right_wall, cube, ceiling, sphere]);
    let from = Point::new(3.0, 8.5, -14.5);
    let to = Point::new(0.0, 0.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    let width = 600;
    let height = 600;
    let fov = PI / 3.5;
    let mut camera = Camera::new(width as f64, height as f64, fov);
    camera.transform = view_transformation(from, to, up);
    let content = camera.render(&world).to_ppm();
    let mut file = File::create("samples/test_1.ppm")?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
