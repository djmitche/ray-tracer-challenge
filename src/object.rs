use crate::{Color, Intersections, Ray, Tup};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::new(1, 1, 1),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

pub trait Object: std::fmt::Debug {
    /// Intersect calculates the intersections of the given ray with this object.
    fn intersect<'o>(&'o self, ray: &Ray) -> Intersections<'o>;

    /// Normal calculates the normal of the given point on the surface of this object.
    fn normal(&self, point: Tup) -> Tup;

    /// Get the object's material
    fn material(&self) -> &Material;
}
