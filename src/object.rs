use crate::{Intersections, Ray, Tup};

pub trait Object: std::fmt::Debug {
    /// Intersect calculates the intersections of the given ray with this object.
    fn intersect<'o>(&'o self, ray: &Ray) -> Intersections<'o>;

    /// Normal calculates the normal of the given point on the surface of this object.
    fn normal(&self, point: Tup) -> Tup;
}
