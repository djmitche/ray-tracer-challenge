use ray_tracer_challenge::*;

fn main() {
    let mut c = Canvas::new(900, 550);
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

    let (width, height) = (c.width(), c.height());
    loop {
        let (ix, iy) = (position.x as usize, position.y as usize);
        if position.y < 0.0 || iy >= height || position.x < 0.0 || ix >= width {
            break;
        }
        c[(position.x as usize, height - position.y as usize)] = color(velocity.magnitude());
        position = position + velocity;
        velocity = velocity + wind + gravity;
    }

    c.write_ppm_file("/tmp/ch2.ppm")
        .expect("could not write PPM file");
}
