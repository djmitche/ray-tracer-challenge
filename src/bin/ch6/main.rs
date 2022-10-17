use ray_tracer_challenge::*;

const SIZE: usize = 1000;
const CAMERA_Z: f64 = -5.0;
const WALL_Z: f64 = 10.0;
const WALL_SIZE: f64 = 7.0;

fn main() {
    let mut c = Canvas::new(SIZE, SIZE);

    let s = Sphere::default()
        .with_transform(Mat::identity().scale(0.75, 0.8, 1))
        .with_material(Material {
            color: Color::new(1, 0.2, 1),
            ..Default::default()
        });
    let origin = Point::new(0, 0, CAMERA_Z);

    let light = Light::new_point(Point::new(-10, 10, -10), Color::white());

    for x in 0..SIZE {
        let wall_x = (x as f64 / SIZE as f64) * WALL_SIZE - WALL_SIZE / 2.0;
        for y in 0..SIZE {
            let wall_y = (y as f64 / SIZE as f64) * WALL_SIZE - WALL_SIZE / 2.0;
            let wall_pt = Point::new(wall_x, -wall_y, WALL_Z);
            let ray = Ray::new(origin, (wall_pt - origin).normalize());

            let mut inters = Intersections::default();
            s.intersect(&ray, &mut inters);
            if let Some(inter) = inters.hit() {
                let hit_pt = ray.position(inter.t);
                let normal = inter.obj.normal(hit_pt);
                let eye = -ray.direction;
                c[(x, y)] = light.lighting(inter.obj.material(), hit_pt, eye, normal, false);
            } else {
                c[(x, y)] = Color::new(0, 0, 1);
            }
        }
    }

    c.write_ppm_file("/tmp/ch6.ppm")
        .expect("could not write PPM file");
}
