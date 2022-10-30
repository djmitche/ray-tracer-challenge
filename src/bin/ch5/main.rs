use image::RgbImage;
use ray_tracer_challenge::*;

const SIZE: u32 = 1000;
const CAMERA_Z: f64 = -5.0;
const WALL_Z: f64 = 10.0;
const WALL_SIZE: f64 = 7.0;

fn main() {
    let mut img = RgbImage::new(SIZE, SIZE);

    let s =
        Object::new(Sphere).with_transform(Mat::identity().scale(1.2, 0.8, 1).translate(0.1, 0, 0));
    let origin = Point::new(0, 0, CAMERA_Z);

    for x in 0..SIZE {
        let wall_x = (x as f64 / SIZE as f64) * WALL_SIZE - WALL_SIZE / 2.0;
        for y in 0..SIZE {
            let wall_y = (y as f64 / SIZE as f64) * WALL_SIZE - WALL_SIZE / 2.0;
            let position = Point::new(wall_x, wall_y, WALL_Z);
            let r = Ray::new(origin, (position - origin).normalize());

            let mut inters = Intersections::default();
            s.intersect(ObjectIndex::test_value(1), &r, &mut inters);
            if let Some(inter) = inters.hit() {
                img.put_pixel(x, y, Color::new((inter.t - 4.0) * 2.0, 1.0, 0).into());
            } else {
                img.put_pixel(x, y, Color::new(0, 0, 1).into());
            }
        }
    }

    img.save("/tmp/ch5.png").expect("could not write PNG file");
}
