use core::f64::consts::PI;
use ray_tracer_challenge::*;

const CAMERA_Z: f64 = -4.0;
const WALL_Z: f64 = 10.0;

fn main() {
    let world = RayMarcher::new(SdfSphere(1.0));
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
