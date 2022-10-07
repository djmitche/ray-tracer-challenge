use crate::Ray;

/// An intersection represents the point at which a ray intersects
/// an object.
#[derive(Debug, Copy, Clone)]
pub struct Intersection<'o> {
    /// The position along the ray at which the intersection occurs
    pub t: f64,

    /// The intersected object
    pub obj: &'o dyn Object,
}

pub trait Object: std::fmt::Debug {
    fn intersect<'o>(&'o self, ray: &Ray) -> Vec<Intersection<'o>>;
}
