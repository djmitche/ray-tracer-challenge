use image::RgbImage;
use ray_tracer_challenge::*;

fn main() {
    let mut img = RgbImage::new(900, 500);
    let start: Point<spaces::World> = Point::new(0, 1, 0);
    let gravity = Vector::new(0, -0.1, 0);
    let wind = Vector::new(-0.02, 0, 0);

    let mut position = start;
    let velocity_mag: f64 = 11.5;
    let mut velocity = Vector::new(1.3, 1.8, 0).normalize() * velocity_mag;

    let stopped: Color = Color::new(1, 1, 0.2);
    let fast: Color = Color::new(0, 0, 1.0);
    let color = |v: f64| {
        let v = v / velocity_mag;
        fast * v + stopped * (1.0 - v)
    };

    let (width, height) = img.dimensions();
    loop {
        let (ix, iy) = (position.x as u32, position.y as u32);
        if position.y < 0.0 || iy >= height || position.x < 0.0 || ix >= width {
            break;
        }
        img.put_pixel(
            position.x as u32,
            height - position.y as u32,
            color(velocity.magnitude()).into(),
        );
        position = position + velocity;
        velocity = velocity + wind + gravity;
    }

    img.save("/tmp/ch2.png").expect("could not write PNG file");
}
