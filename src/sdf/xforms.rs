use crate::sdf::Sdf;
use crate::{spaces, Point};

pub struct Rounded<SDF: Sdf> {
    radius: f64,
    inner: SDF,
}

impl<SDF: Sdf> Rounded<SDF> {
    pub fn new(inner: SDF, radius: f64) -> Self {
        Self { radius, inner }
    }
}

impl<SDF: Sdf> Sdf for Rounded<SDF> {
    fn distance(&self, point: &Point<spaces::World>) -> f64 {
        self.inner.distance(point) - self.radius
    }
}
