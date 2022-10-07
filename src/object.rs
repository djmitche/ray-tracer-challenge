use crate::Intersections;
use crate::Ray;

pub trait Object: std::fmt::Debug {
    fn intersect<'o>(&'o self, ray: &Ray) -> Intersections<'o>;
}
