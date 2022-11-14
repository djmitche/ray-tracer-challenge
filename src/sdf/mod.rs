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
        let TINY_X: Vector<spaces::World> = Vector::new(0.00001, 0, 0);
        let TINY_Y: Vector<spaces::World> = Vector::new(0, 0.00001, 0);
        let TINY_Z: Vector<spaces::World> = Vector::new(0, 0, 0.00001);

        let grad_x = self.sdf.distance(&(point + TINY_X)) - self.sdf.distance(&(point - TINY_X));
        let grad_y = self.sdf.distance(&(point + TINY_Y)) - self.sdf.distance(&(point - TINY_Y));
        let grad_z = self.sdf.distance(&(point + TINY_Z)) - self.sdf.distance(&(point - TINY_Z));

        Vector::new(grad_x, grad_y, grad_z).normalize()
    }
}

impl<SDF: Sdf + Send + Sync> RayColor for RayMarcher<SDF> {
    fn color_at(&self, ray: &Ray<spaces::World>, _debug: bool) -> Color {
        let mut total_distance = 0.0;
        const MAX_STEPS: u32 = 32;
        const MAX_DISTANCE: f64 = 1000.0;
        const EPSILON: f64 = 0.001;

        let LIGHT_POS: Point<spaces::World> = Point::new(2, -5, -10);

        for _ in 0..MAX_STEPS {
            let pos = ray.position(total_distance);
            let dist = self.sdf.distance(&pos);

            if dist < EPSILON {
                let normal = self.normal(pos);
                let direction_to_light = (LIGHT_POS - pos).normalize();
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
