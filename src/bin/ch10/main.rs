use core::f64::consts::PI;
use ray_tracer_challenge::*;

const SIZE: usize = 500;
const CAMERA_Z: f64 = -5.0;
const WALL_Z: f64 = 10.0;

fn main() {
    let mut world = World::default();
    world.light = Light::new_point(Point::new(-10, 10, -10), Color::white());

    world.add_object(
        Object::new(Sphere)
            .with_transform(Mat::identity().scale(0.75, 0.8, 1))
            .with_material(Material {
                pattern: Pattern::stripe(Color::white(), Color::new(1, 0.5, 0.5)),
                ambient: 0.2,
                ..Default::default()
            }),
    );
    world.add_object(
        Object::new(Sphere)
            .with_transform(Mat::identity().scale(0.5, 0.5, 0.5).translate(-0.75, 0, -1))
            .with_material(Material {
                pattern: Pattern::gradient(Color::white(), Color::new(1, 0.5, 0.5)).with_transform(
                    Mat::identity()
                        .scale(1, 1, 1)
                        .rotate_x(PI / 2.0)
                        .rotate_z(PI / 9.0),
                ),
                ambient: 0.2,
                ..Default::default()
            }),
    );
    world.add_object(
        Object::new(Sphere)
            .with_transform(
                Mat::identity()
                    .scale(0.5, 0.5, 0.5)
                    .translate(0.75, 0, -0.9),
            )
            .with_material(Material {
                pattern: Pattern::checker(Color::white(), Color::new(1, 0.5, 0.5))
                    .with_transform(Mat::identity().scale(0.1, 0.1, 0.1).rotate_z(PI / 4.0)),
                ambient: 0.2,
                ..Default::default()
            }),
    );

    world.add_object(
        Object::new(Plane)
            .with_transform(Mat::identity().translate(0, -0.5, 0))
            .with_material(Material {
                pattern: Pattern::stripe_of(
                    Pattern::checker(Color::white(), Color::new(0.8, 0.8, 0.0))
                        .with_transform(Mat::identity().rotate_y(PI / 3.0)),
                    Pattern::checker(Color::white(), Color::new(0.0, 0.8, 0.8))
                        .with_transform(Mat::identity().rotate_y(PI / 3.0)),
                ),
                ambient: 0.2,
                ..Default::default()
            }),
    );

    let camera = Camera::new(
        SIZE,
        SIZE,
        PI / 3.0,
        Point::new(1.5, 2, CAMERA_Z),
        Point::new(-2, -1, WALL_Z),
        Vector::new(0, 1, 0),
        3,
    );

    camera
        .render(&world)
        .write_ppm_file("/tmp/ch10.ppm")
        .expect("could not write PPM file");
}
