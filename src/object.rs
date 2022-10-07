use crate::Ray;

pub trait Object {
    fn intersect(&self, ray: &Ray) -> Vec<f64>;
}
