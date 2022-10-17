use ray_tracer_challenge::*;

const SIZE: usize = 500;
const CAMERA_Z: f64 = -5.0;
const WALL_Z: f64 = 10.0;
const WALL_SIZE: f64 = 7.0;

fn main() {
    let mut c = Canvas::new(SIZE, SIZE);

    let mut world = World::default();
    world.light = Light::new_point(Point::new(-10, 10, -10), Color::white());

    world.add(Box::new(
        Sphere::default()
            .with_transform(Mat::identity().scale(0.75, 0.8, 1))
            .with_material(Material {
                color: Color::new(1, 0.2, 1),
                ambient: 0.2,
                ..Default::default()
            }),
    ));
    world.add(Box::new(
        Sphere::default()
            .with_transform(Mat::identity().scale(0.5, 0.5, 0.5).translate(-0.75, 0, -1))
            .with_material(Material {
                color: Color::new(0.3, 0.8, 0.1),
                ambient: 0.2,
                ..Default::default()
            }),
    ));
    world.add(Box::new(
        Sphere::default()
            .with_transform(
                Mat::identity()
                    .scale(0.5, 0.5, 0.5)
                    .translate(0.75, 0, -0.9),
            )
            .with_material(Material {
                color: Color::new(0.8, 0.8, 0.1),
                ambient: 0.2,
                ..Default::default()
            }),
    ));
    let origin = Point::new(0, 0, CAMERA_Z);

    for x in 0..SIZE {
        let wall_x = (x as f64 / SIZE as f64) * WALL_SIZE - WALL_SIZE / 2.0;
        for y in 0..SIZE {
            let wall_y = (y as f64 / SIZE as f64) * WALL_SIZE - WALL_SIZE / 2.0;
            let wall_pt = Point::new(wall_x, -wall_y, WALL_Z);
            let ray = Ray::new(origin, (wall_pt - origin).normalize());

            c[(x, y)] = world.color_at(&ray);
        }
    }

    c.write_ppm_file("/tmp/ch7.ppm")
        .expect("could not write PPM file");
}
