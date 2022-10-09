use ray_tracer_challenge::*;

const SIZE: usize = 300;
const CAMERA_Z: f64 = -5.0;
const WALL_Z: f64 = 10.0;
const WALL_SIZE: f64 = 7.0;

fn main() {
    let mut c = Canvas::new(SIZE, SIZE);

    let s = Sphere::with_transform(Mat::identity().scale(1.2, 0.8, 1).translate(0.1, 0, 0));
    let origin = Tup::point(0, 0, CAMERA_Z);

    for x in 0..SIZE {
        let wall_x = (x as f64 / SIZE as f64) * WALL_SIZE - WALL_SIZE / 2.0;
        for y in 0..SIZE {
            let wall_y = (y as f64 / SIZE as f64) * WALL_SIZE - WALL_SIZE / 2.0;
            let position = Tup::point(wall_x, wall_y, WALL_Z);
            let r = Ray::new(origin, (position - origin).normalize());

            if let Some(inter) = s.intersect(&r).hit() {
                c[(x, y)] = Color::new((inter.t - 4.0) * 2.0, 1.0, 0);
            } else {
                c[(x, y)] = Color::new(0, 0, 1);
            }
        }
    }

    c.write_ppm_file("/tmp/ch5.ppm")
        .expect("could not write PPM file");
}
