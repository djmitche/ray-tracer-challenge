use core::f64::consts::PI;
use ray_tracer_challenge::*;

const CAMERA_Z: f64 = -5.0;
const WALL_Z: f64 = 10.0;

fn main() {
    let mut world = World::default();
    world.set_light(Light::new_point(Point::new(-10, 10, -10), Color::white()));

    world.add_object(
        Object::new(Sphere)
            .with_transform(Mat::identity().scale(0.75, 0.8, 1))
            .with_material(
                Material::default()
                    .with_pattern(Pattern::stripe(Color::white(), Color::new(1, 0.5, 0.5)))
                    .with_ambient(0.2)
                    .with_reflectivity(0.2),
            ),
    );
    world.add_object(
        Object::new(Sphere)
            .with_transform(Mat::identity().scale(0.5, 0.5, 0.5).translate(-0.75, 0, -1))
            .with_material(
                Material::default()
                    .with_pattern(
                        Pattern::gradient(Color::white(), Color::new(1, 0.5, 0.5)).with_transform(
                            Mat::identity()
                                .scale(1, 1, 1)
                                .rotate_x(PI / 2.0)
                                .rotate_z(PI / 9.0),
                        ),
                    )
                    .with_ambient(0.2)
                    .with_reflectivity(0.2),
            ),
    );
    world.add_object(
        Object::new(Sphere)
            .with_transform(
                Mat::identity()
                    .scale(0.5, 0.5, 0.5)
                    .translate(0.75, 0, -0.9),
            )
            .with_material(
                Material::default()
                    .with_pattern(
                        Pattern::checker(Color::white(), Color::new(1, 0.5, 0.5)).with_transform(
                            Mat::identity().scale(0.1, 0.1, 0.1).rotate_z(PI / 4.0),
                        ),
                    )
                    .with_ambient(0.2)
                    .with_reflectivity(0.2),
            ),
    );

    world.add_object(
        Object::new(Plane)
            .with_transform(Mat::identity().translate(0, -0.5, 0))
            .with_material(
                Material::default()
                    .with_pattern(Pattern::blend(
                        Pattern::checker(Color::black(), Color::new(0.8, 0.8, 0.0)).with_transform(
                            Mat::identity().translate(0, -0.1, 0).rotate_y(PI / 3.0),
                        ),
                        Pattern::gradient(Color::black(), Color::new(0.0, 0.8, 0.8))
                            .with_transform(
                                Mat::identity()
                                    .translate(-0.5, 0, 0)
                                    .scale(10, 1, 1)
                                    .rotate_y(PI / 3.4),
                            ),
                    ))
                    .with_ambient(0.2)
                    .with_reflectivity(0.5),
            ),
    );

    display(
        world,
        PI / 3.0,
        Point::new(1.5, 2, CAMERA_Z),
        Point::new(-2, -1, WALL_Z),
        Vector::new(0, 1, 0),
        1,
    );
}
