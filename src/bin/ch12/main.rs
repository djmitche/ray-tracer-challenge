use core::f64::consts::PI;
use ray_tracer_challenge::csg::*;
use ray_tracer_challenge::*;

const CAMERA_Z: f64 = -4.0;
const WALL_Z: f64 = 10.0;

fn main() {
    let mut world = World::default();
    world.set_light(Light::new_point(Point::new(-10, 10, -10), Color::white()));

    macro_rules! cube {
        ($color:expr, $x:expr, $z:expr) => {
            world.add_object(
                Object::new(Cube)
                    .with_material(
                        Material::default()
                            .with_color($color)
                            .with_ambient(0.5)
                            .with_diffuse(0.5)
                            .with_specular(0.6)
                            .with_reflectivity(0.4)
                            .with_transparency(0.99, 1.1),
                    )
                    .with_transform(Mat::identity().translate($x, 1.3, $z)),
            );
        };
    }

    cube!(Color::new(0.1, 0, 0), 0, 0);
    cube!(Color::new(0, 0.1, 0), 2, 0);
    cube!(Color::new(0, 0, 0.1), -2, 0);

    world.add_object(
        Object::new(Plane).with_material(
            Material::default()
                .with_pattern(
                    Pattern::checker(Color::black(), Color::new(0.8, 0.8, 0.0))
                        .with_transform(Mat::identity().translate(0, 0.1, 0)),
                )
                .with_ambient(0.2),
        ),
    );

    display(
        world,
        PI / 2.0,
        Point::new(0, 2, CAMERA_Z),
        Point::new(0, -1, WALL_Z),
        Vector::new(0, 1, 0),
        1,
    );
}
