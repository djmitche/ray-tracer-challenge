use crate::Tup;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    pub center: Tup,
    pub radius: f64,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Tup::point(0, 0, 0),
            radius: 1.0,
        }
    }
}
