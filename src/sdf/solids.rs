use crate::sdf::Sdf;
use crate::{spaces, Point, Ray};

pub struct Sphere(f64);

impl Sphere {
    pub fn new(radius: f64) -> Self {
        Sphere(radius)
    }
}

impl Sdf for Sphere {
    fn distance(&self, point: &Point<spaces::World>) -> f64 {
        let radius = self.0;
        point.as_vector().magnitude() - radius
    }
}

pub struct Line {
    ray: Ray<spaces::World>,
}

impl Line {
    pub fn new(ray: Ray<spaces::World>) -> Self {
        Line { ray }
    }
}

impl Sdf for Line {
    fn distance(&self, point: &Point<spaces::World>) -> f64 {
        // compute the t of the nearest point on the ray
        let t = (point.as_vector().dot(self.ray.direction)
            - self.ray.origin.as_vector().dot(self.ray.direction))
            / (self.ray.direction.dot(self.ray.direction));
        let intersection = self.ray.position(t);
        (*point - intersection).magnitude()
    }
}
