use core::f64::consts::PI;
use ray_tracer_challenge::*;
use rayon::prelude::*;

const SIZE: usize = 2000;

fn main() {
    let mut canvas = Canvas::new(SIZE, SIZE);

    let mut world = World::default();
    world.light = Light::new_point(Tup::point(-10, 10, -10), Color::new(1, 1, 1));

    // floor
    world.add(Box::new(
        Sphere::default()
            .with_transform(Mat::identity().scale(10, 0.01, 10))
            .with_material(Material {
                specular: 0.0,
                ..Default::default()
            }),
    ));

    // left_wall
    world.add(Box::new(
        Sphere::default()
            .with_transform(
                Mat::identity()
                    .scale(10, 0.01, 10)
                    .rotate_x(PI / 2.0)
                    .rotate_y(-PI / 4.0)
                    .translate(0, 0, 5),
            )
            .with_material(Material {
                specular: 0.0,
                ..Default::default()
            }),
    ));

    // right_wall
    world.add(Box::new(
        Sphere::default()
            .with_transform(
                Mat::identity()
                    .scale(10, 0.01, 10)
                    .rotate_x(PI / 2.0)
                    .rotate_y(PI / 4.0)
                    .translate(0, 0, 5),
            )
            .with_material(Material {
                specular: 0.0,
                ..Default::default()
            }),
    ));

    // middle
    world.add(Box::new(
        Sphere::default()
            .with_transform(Mat::identity().translate(-0.5, 1, 0.5))
            .with_material(Material {
                color: Color::new(0.1, 1, 0.5),
                diffuse: 0.7,
                specular: 0.3,
                ..Default::default()
            }),
    ));

    // right
    world.add(Box::new(
        Sphere::default()
            .with_transform(
                Mat::identity()
                    .scale(0.5, 0.5, 0.5)
                    .translate(1.5, 0.5, -0.5),
            )
            .with_material(Material {
                color: Color::new(0.5, 1, 0.1),
                diffuse: 0.7,
                specular: 0.3,
                ..Default::default()
            }),
    ));

    // left
    world.add(Box::new(
        Sphere::default()
            .with_transform(
                Mat::identity()
                    .scale(0.33, 0.33, 0.33)
                    .translate(-1.5, 0.33, -0.75),
            )
            .with_material(Material {
                color: Color::new(1, 0.8, 0.1),
                diffuse: 0.7,
                specular: 0.3,
                ..Default::default()
            }),
    ));

    let camera = Camera::new(
        SIZE,
        SIZE,
        PI / 3.0,
        Tup::point(0, 1.9, -5),
        Tup::point(0, 1, 0),
        Tup::vector(0, 1, 0),
    );

    let camref = &camera;
    let worldref = &world;
    for (x, y, c) in camref
        .into_iter()
        .par_bridge()
        .map(move |(x, y)| (x, y, camref.color_at(x, y, worldref)))
        .collect::<Vec<_>>()
    {
        canvas[(x, y)] = c;
    }

    canvas
        .write_ppm_file(&format!("/tmp/ch7b.ppm"))
        .expect("could not write PPM file");
}