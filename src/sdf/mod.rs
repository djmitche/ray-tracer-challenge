pub use crate::*;

pub trait Sdf {
    fn distance(&self, point: &Point<spaces::World>) -> f64;
}

pub struct RayMarcher<SDF: Sdf + Send + Sync> {
    sdf: SDF,
}

impl<SDF: Sdf + Send + Sync> RayMarcher<SDF> {
    pub fn new(sdf: SDF) -> Self {
        RayMarcher { sdf }
    }

    fn normal(&self, point: Point<spaces::World>) -> Vector<spaces::World> {
        let tiny_x: Vector<spaces::World> = Vector::new(0.00001, 0, 0);
        let tiny_y: Vector<spaces::World> = Vector::new(0, 0.00001, 0);
        let tiny_z: Vector<spaces::World> = Vector::new(0, 0, 0.00001);

        let grad_x = self.sdf.distance(&(point + tiny_x)) - self.sdf.distance(&(point - tiny_x));
        let grad_y = self.sdf.distance(&(point + tiny_y)) - self.sdf.distance(&(point - tiny_y));
        let grad_z = self.sdf.distance(&(point + tiny_z)) - self.sdf.distance(&(point - tiny_z));

        Vector::new(grad_x, grad_y, grad_z).normalize()
    }
}

impl<SDF: Sdf + Send + Sync> RayColor for RayMarcher<SDF> {
    fn color_at(&self, ray: &Ray<spaces::World>, _debug: bool) -> Color {
        let mut total_distance = 0.0;
        const MAX_STEPS: u32 = 32;
        const MAX_DISTANCE: f64 = 1000.0;
        const EPSILON: f64 = 0.001;

        let light_pos: Point<spaces::World> = Point::new(2, -5, -10);

        for _ in 0..MAX_STEPS {
            let pos = ray.position(total_distance);
            let dist = self.sdf.distance(&pos);

            if dist < EPSILON {
                let normal = self.normal(pos);
                let direction_to_light = (light_pos - pos).normalize();
                let diffuse_intensity = normal.dot(direction_to_light);
                return Color::new(diffuse_intensity, diffuse_intensity, diffuse_intensity);
            }

            total_distance += dist;
            if total_distance > MAX_DISTANCE {
                break;
            }
        }
        Color::black()
    }
}

pub struct SdfSphere(pub f64);

impl Sdf for SdfSphere {
    fn distance(&self, point: &Point<spaces::World>) -> f64 {
        let radius = self.0;
        point.as_vector().magnitude() - radius
    }
}
