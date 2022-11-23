use core::f64::consts::PI;
use ray_tracer_challenge::sdf::*;
use ray_tracer_challenge::*;

const CAMERA_Z: f64 = -4.0;
const WALL_Z: f64 = 10.0;

fn main() {
    let world = RayMarcher::new(intersection!(
        Sphere::new(1.0),
        union!(
            Rounded::new(
                Line::new(Ray::new(
                    Point::new(0, 0, 0),
                    Vector::new(1, 1, -1).normalize(),
                )),
                0.1,
            ),
            Rounded::new(
                Line::new(Ray::new(
                    Point::new(0, 0, 0),
                    Vector::new(1, 0, -1).normalize(),
                )),
                0.1,
            ),
        ),
    ));
    //world.set_light(Light::new_point(Point::new(-10, 10, -10), Color::white()));

    display(
        world,
        PI / 2.0,
        Point::new(0, 2, CAMERA_Z),
        Point::new(0, -1, WALL_Z),
        Vector::new(0, 1, 0),
        1,
    );
}
