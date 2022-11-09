use core::f64::consts::PI;
use ray_tracer_challenge::*;

const CAMERA_Z: f64 = -4.0;
const WALL_Z: f64 = 10.0;

fn main() {
    let mut world = World::default();
    world.set_light(Light::new_point(Point::new(-10, 10, -10), Color::white()));

    /*
    for x in -2..2 {
        for z in -2..2 {
            world.add_object(
                Object::new(Sphere)
                    .with_transform(Mat::identity().scale(0.25, 0.25, 0.25).translate(x, 1.0, z))
                    .with_material(
                        Material::default()
                            .with_color(Color::new(1, 0.5, 0.5))
                            .with_ambient(0.1)
                            .with_diffuse(0.1)
                            .with_specular(0.0)
                            //.with_reflectivity(0.1),
                            .with_transparency(1.0, 1.0),
                    ),
            );
        }
    }
    */
    world.add_object(
        Object::new(Sphere).with_material(
            Material::default()
                .with_color(Color::white())
                .with_ambient(0.05)
                .with_diffuse(0.05)
                .with_specular(0.6)
                .with_transparency(1.0, 1.3),
        ),
    );

    world.add_object(
        Object::new(Plane).with_material(
            Material::default()
                .with_pattern(
                    Pattern::checker(Color::black(), Color::new(0.8, 0.8, 0.0))
                        .with_transform(Mat::identity().translate(0, -0.1, 0).rotate_y(PI / 3.0)),
                )
                .with_ambient(0.2),
        ),
    );

    display(
        world,
        PI / 2.0,
        Point::new(0.5, 2, CAMERA_Z),
        Point::new(-0.2, -1, WALL_Z),
        Vector::new(0, 1, 0),
        1,
    );
}
