use ray_tracer_challenge::*;
use std::f64::consts::PI;

fn main() {
    let mut c = Canvas::new(100, 100);

    for h in 0..12 {
        let m: Mat<4, spaces::World, spaces::World> = Mat::identity()
            .rotate_z((h as f64) * PI / 6.0)
            .scale(45, 45, 0)
            .translate(50, 50, 0);

        let p = m * Point::new(0, 1, 0);
        c[(p.x as usize, p.y as usize)] = Color::new(1, 1, 1);
    }

    c.write_ppm_file("/tmp/ch4.ppm")
        .expect("could not write PPM file");
}
