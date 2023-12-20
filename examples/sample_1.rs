use std::{f64::consts::PI, fs::File, io::Write};

use rt_new::features::{
    rotation_x, rotation_z, scaling, translation, view_transformation, Camera, Color, Light,
    Material, Object, Pattern, Point, Vector, World, GREY, WHITE,
};

fn main() -> std::io::Result<()> {
    let floor = Object::plane_builder()
        .material(
            Material::builder()
                .pattern(Pattern::checker(WHITE, GREY))
                .build(),
        )
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
    let sphere_1 = Object::sphere_builder()
        .material(Material::builder().color(Color::new(0.2, 1.0, 0.3)).build())
        .transformation(translation(-6.0, 2.0, 2.0) * scaling(2.0, 2.0, 2.0))
        .build();
    let sphere_2 = Object::sphere_builder()
        .material(Material::builder().reflective(0.9).build())
        .transformation(translation(-5.0, 2.0, -3.0) * scaling(2.0, 2.0, 2.0))
        .build();
    let sphere_3 = Object::sphere_builder()
        .material(
            Material::builder()
                .color(Color::new(0.1, 0.1, 1.0))
                .diffuse(0.8)
                .build(),
        )
        .transformation(translation(-2.0, 2.0, 2.0) * scaling(2.0, 2.0, 2.0))
        .build();
    let sphere_4 = Object::sphere_builder()
        .material(
            Material::builder()
                .color(Color::new(1.0, 0.1, 0.1))
                .diffuse(0.8)
                .specular(0.7)
                .build(),
        )
        .transformation(translation(0.0, 2.0, -2.5) * scaling(2.0, 2.0, 2.0))
        .build();
    let light = Light::new(Point::new(-5.0, 10.0, -10.0), WHITE);
    let mut world = World::new(light);
    world.add_shapes(vec![
        floor, left_wall, right_wall, sphere_1, sphere_2, sphere_3, sphere_4, ceiling,
    ]);
    let from = Point::new(3.0, 8.5, -14.5);
    let to = Point::new(0.0, 0.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    let width = 1200;
    let height = 1200;
    let fov = PI / 3.5;
    let mut camera = Camera::new(width as f64, height as f64, fov);
    camera.transform = view_transformation(from, to, up);
    let content = camera.render(&world).to_ppm();
    let mut file = File::create("samples/sample_1.ppm")?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
