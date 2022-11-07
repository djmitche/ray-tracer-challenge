use core::f64::consts::PI;
use ray_tracer_challenge::*;

const SIZE: u32 = 2000;

fn main() {
    let mut world = World::default();
    world.set_light(Light::new_point(
        Point::new(-10, 10, -10),
        Color::new(1, 1, 1),
    ));

    // floor
    world.add_object(
        Object::new(Sphere)
            .with_transform(Mat::identity().scale(10, 0.01, 10))
            .with_material(Material::default().with_specular(0.0)),
    );

    // left_wall
    world.add_object(
        Object::new(Sphere)
            .with_transform(
                Mat::identity()
                    .scale(10, 0.01, 10)
                    .rotate_x(PI / 2.0)
                    .rotate_y(-PI / 4.0)
                    .translate(0, 0, 5),
            )
            .with_material(Material::default().with_specular(0.0)),
    );

    // right_wall
    world.add_object(
        Object::new(Sphere)
            .with_transform(
                Mat::identity()
                    .scale(10, 0.01, 10)
                    .rotate_x(PI / 2.0)
                    .rotate_y(PI / 4.0)
                    .translate(0, 0, 5),
            )
            .with_material(Material::default().with_specular(0.0)),
    );

    // middle
    world.add_object(
        Object::new(Sphere)
            .with_transform(Mat::identity().translate(-0.5, 1, 0.5))
            .with_material(
                Material::default()
                    .with_color(Color::new(0.1, 1, 0.5))
                    .with_diffuse(0.7)
                    .with_specular(0.3),
            ),
    );

    // right
    world.add_object(
        Object::new(Sphere)
            .with_transform(
                Mat::identity()
                    .scale(0.5, 0.5, 0.5)
                    .translate(1.5, 0.5, -0.5),
            )
            .with_material(
                Material::default()
                    .with_color(Color::new(0.5, 1, 0.1))
                    .with_diffuse(0.7)
                    .with_specular(0.3),
            ),
    );

    // left
    world.add_object(
        Object::new(Sphere)
            .with_transform(
                Mat::identity()
                    .scale(0.33, 0.33, 0.33)
                    .translate(-1.5, 0.33, -0.75),
            )
            .with_material(
                Material::default()
                    .with_color(Color::new(1, 0.8, 0.1))
                    .with_diffuse(0.7)
                    .with_specular(0.3),
            ),
    );

    let camera = Camera::new(
        SIZE,
        SIZE,
        PI / 3.0,
        Point::new(0, 1.9, -5),
        Point::new(0, 1, 0),
        Vector::new(0, 1, 0),
        3,
    );

    camera
        .render(&world)
        .save("/tmp/ch7b.png")
        .expect("could not write PNG file");
}
