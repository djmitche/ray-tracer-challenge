use super::{Point, Space};
use approx::{relative_eq, AbsDiffEq, RelativeEq};
use std::marker::PhantomData;

/// Vector represents a location in the given space.
///
/// This can be represented as a 3-tuple with labels x, y, z.  In 4-dimensional transformation, it
/// acts as though it has a fourth element with value 0.0.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Vector<S: Space> {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    space: PhantomData<S>,
}

impl<S: Space> Vector<S> {
    /// Create a new tuple.
    pub fn new<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(x: X, y: Y, z: Z) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            space: PhantomData,
        }
    }

    /// Convert a vector to a point.
    pub fn as_point(&self) -> Point<S> {
        Point::new(self.x, self.y, self.z)
    }

    /// Determine the magnitude of this vector
    pub fn magnitude(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    /// Return a scaled vector with magnitude 1.0.  This will fail for a zero tuple.
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Vector {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            space: PhantomData,
        }
    }

    /// Compute the dot product of two vectors
    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Compute the cross product of two tuples
    pub fn cross(&self, other: Self) -> Self {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Reflect this vector to the opposite side of the given normal.
    pub fn reflect(&self, normal: Vector<S>) -> Self {
        *self - normal * 2.0 * self.dot(normal)
    }
}

/*
impl<S: Space> std::ops::Index<usize> for Vector<S> {
    type Output = f64;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &0.0,
            _ => unreachable!(),
        }
    }
}

impl<S: Space> std::ops::IndexMut<usize> for Vector<S> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => panic!("cannot modify w coordinate of a point"),
            _ => unreachable!(),
        }
    }
}
*/

impl<S: Space> AbsDiffEq for Vector<S> {
    type Epsilon = <f64 as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> <f64 as AbsDiffEq>::Epsilon {
        <f64 as AbsDiffEq>::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: <f64 as AbsDiffEq>::Epsilon) -> bool {
        <f64 as AbsDiffEq>::abs_diff_eq(&self.x, &other.x, epsilon)
            && <f64 as AbsDiffEq>::abs_diff_eq(&self.y, &other.y, epsilon)
            && <f64 as AbsDiffEq>::abs_diff_eq(&self.z, &other.z, epsilon)
    }
}

impl<S: Space> RelativeEq for Vector<S> {
    fn default_max_relative() -> <f64 as AbsDiffEq>::Epsilon {
        <f64 as RelativeEq>::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: <f64 as AbsDiffEq>::Epsilon,
        max_relative: <f64 as AbsDiffEq>::Epsilon,
    ) -> bool {
        <f64 as RelativeEq>::relative_eq(&self.x, &other.x, epsilon, max_relative)
            && <f64 as RelativeEq>::relative_eq(&self.y, &other.y, epsilon, max_relative)
            && <f64 as RelativeEq>::relative_eq(&self.z, &other.z, epsilon, max_relative)
    }
}

impl<S: Space> std::ops::Add for Vector<S> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            space: PhantomData,
        }
    }
}

impl<S: Space> std::ops::Sub for Vector<S> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            space: PhantomData,
        }
    }
}

impl<S: Space> std::ops::Neg for Vector<S> {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            space: PhantomData,
        }
    }
}

impl<S: Space> std::ops::Mul<f64> for Vector<S> {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            space: PhantomData,
        }
    }
}

impl<S: Space> std::ops::Div<f64> for Vector<S> {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            space: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::spaces;
    use approx::*;

    #[test]
    fn vector() {
        let a: Vector<spaces::World> = Vector::new(4.3, -4.2, 3.1);
        assert_relative_eq!(a.x, 4.3);
        assert_relative_eq!(a.y, -4.2);
        assert_relative_eq!(a.z, 3.1);
    }

    #[test]
    fn as_point() {
        let a: Vector<spaces::World> = Vector::new(4.3, -4.2, 3.1);
        let b: Point<spaces::World> = a.as_point();
        assert_relative_eq!(b.x, 4.3);
        assert_relative_eq!(b.y, -4.2);
        assert_relative_eq!(b.z, 3.1);
    }

    #[test]
    fn adding_two_vectors() {
        let a1: Vector<spaces::World> = Vector::new(3.0, -2.0, 5.0);
        let a2 = Vector::new(-2.0, 3.0, 1.0);
        assert_relative_eq!(a1 + a2, Vector::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1: Vector<spaces::World> = Vector::new(3, 2, 1);
        let v2 = Vector::new(5, 6, 7);
        assert_relative_eq!(v1 - v2, Vector::new(-2, -4, -6));
    }

    /// Negating a vector
    #[test]
    fn negating_vector() {
        let a: Vector<spaces::World> = Vector::new(1.0, -2.0, 3.0);
        assert_relative_eq!(-a, Vector::new(-1.0, 2.0, -3.0));
    }

    /// Multiplying a vector by a scalar
    #[test]
    fn mult_vec_by_scalar() {
        let a: Vector<spaces::World> = Vector::new(1.0, -2.0, 3.0);
        assert_relative_eq!(a * 3.5, Vector::new(3.5, -7.0, 10.5));
    }

    /// Multiplying a vector by a fraction
    #[test]
    fn mult_vec_by_fraction() {
        let a: Vector<spaces::World> = Vector::new(1.0, -2.0, 3.0);
        assert_relative_eq!(a * 0.5, Vector::new(0.5, -1.0, 1.5));
    }

    /// Dividing a vector by a scalar
    #[test]
    fn div_vec_by_scalar() {
        let a: Vector<spaces::World> = Vector::new(1.0, -2.0, 3.0);
        assert_relative_eq!(a / 2.0, Vector::new(0.5, -1.0, 1.5));
    }

    /// Computing the magnitude of a vector(1, 0, 0)
    #[test]
    fn vec_magnitude_100() {
        let v: Vector<spaces::World> = Vector::new(1, 0, 0);
        assert_relative_eq!(v.magnitude(), 1.0);
    }

    /// Computing the magnitude of a vector(0, 1, 0)
    #[test]
    fn vec_magnitude_010() {
        let v: Vector<spaces::World> = Vector::new(0, 1, 0);
        assert_relative_eq!(v.magnitude(), 1.0);
    }

    /// Computing the magnitude of a vector(0, 0, 1)
    #[test]
    fn vec_magnitude_001() {
        let v: Vector<spaces::World> = Vector::new(0, 0, 1);
        assert_relative_eq!(v.magnitude(), 1.0);
    }

    /// Computing the magnitude of a vector(1, 2, 3)
    #[test]
    fn vec_magnitude_123() {
        let v: Vector<spaces::World> = Vector::new(1, 2, 3);
        assert_relative_eq!(v.magnitude(), 14f64.sqrt());
    }

    /// Computing the magnitude of a vector(-1, -2, -3)
    #[test]
    fn vec_magnitude_neg_123() {
        let v: Vector<spaces::World> = Vector::new(-1, -2, -3);
        assert_relative_eq!(v.magnitude(), 14f64.sqrt());
    }

    /// Normalizing vector(4, 0, 0) gives (1, 0, 0)
    #[test]
    fn vec_normalize_400() {
        let v: Vector<spaces::World> = Vector::new(4, 0, 0);
        assert_relative_eq!(v.normalize(), Vector::new(1, 0, 0));
    }

    /// Normalizing vector(1, 2, 3)
    #[test]
    fn vec_normalize_123() {
        let v: Vector<spaces::World> = Vector::new(1, 2, 3);
        assert_relative_eq!(
            v.normalize(),
            Vector::new(1.0 / 14f64.sqrt(), 2.0 / 14f64.sqrt(), 3.0 / 14f64.sqrt())
        );
    }

    /// Magnitude of a normalized vector
    #[test]
    fn vec_normalized_magnitude() {
        let v: Vector<spaces::World> = Vector::new(1, 2, 3);
        assert_relative_eq!(v.normalize().magnitude(), 1.0);
    }

    /// The cross product of two vectors
    #[test]
    fn cross_product() {
        let a: Vector<spaces::World> = Vector::new(1, 2, 3);
        let b = Vector::new(2, 3, 4);
        assert_relative_eq!(a.cross(b), Vector::new(-1, 2, -1));
        assert_relative_eq!(b.cross(a), Vector::new(1, -2, 1));
    }

    /// The dot product of two tuples
    #[test]
    fn dot_product() {
        let a: Vector<spaces::World> = Vector::new(1, 2, 3);
        let b = Vector::new(2, 3, 4);
        assert_relative_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn reflect_approaching_at_45() {
        let v: Vector<spaces::World> = Vector::new(1, -1, 0);
        let n = Vector::new(0, 1, 0);
        assert_relative_eq!(v.reflect(n), Vector::new(1, 1, 0));
    }

    #[test]
    fn reflect_slanted() {
        let v: Vector<spaces::World> = Vector::new(0, -1, 0);
        let n = Vector::new(2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0, 0);
        assert_relative_eq!(v.reflect(n), Vector::new(1, 0, 0));
    }
}
