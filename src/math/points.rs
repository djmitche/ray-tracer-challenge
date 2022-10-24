use super::{Space, Vector};
use approx::{AbsDiffEq, RelativeEq};
use std::marker::PhantomData;

/// Point represents a location in the given space.
///
/// This can be represented as a 3-tuple with labels x, y, z.  In 4-dimensional transformation, it
/// acts as though it has a fourth element with value 1.0.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Point<S: Space> {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    space: PhantomData<S>,
}

impl<S: Space> Point<S> {
    /// Create a new point.
    pub fn new<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(x: X, y: Y, z: Z) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            space: PhantomData,
        }
    }

    /// Convert a point to a vector
    pub fn as_vector(&self) -> Vector<S> {
        Vector::new(self.x, self.y, self.z)
    }

    /// Convert the point to another space, in-place
    pub fn as_space<S2: Space>(&self) -> Point<S2> {
        Point {
            x: self.x,
            y: self.y,
            z: self.z,
            space: PhantomData,
        }
    }
}

impl<S: Space> AbsDiffEq for Point<S> {
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

impl<S: Space> RelativeEq for Point<S> {
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

impl<S: Space> std::ops::Add<Vector<S>> for Point<S> {
    type Output = Self;
    fn add(self, other: Vector<S>) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            space: PhantomData,
        }
    }
}

impl<S: Space> std::ops::Sub<Vector<S>> for Point<S> {
    type Output = Self;
    fn sub(self, other: Vector<S>) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            space: PhantomData,
        }
    }
}

impl<S: Space> std::ops::Sub<Point<S>> for Point<S> {
    type Output = Vector<S>;
    fn sub(self, other: Point<S>) -> Self::Output {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::spaces;
    use approx::*;

    #[test]
    fn point() {
        let a: Point<spaces::World> = Point::new(4.3, -4.2, 3.1);
        assert_relative_eq!(a.x, 4.3);
        assert_relative_eq!(a.y, -4.2);
        assert_relative_eq!(a.z, 3.1);
    }

    #[test]
    fn as_vector() {
        let a: Point<spaces::World> = Point::new(3.0, -2.0, 5.0);
        let b: Vector<spaces::World> = a.as_vector();
        assert_relative_eq!(b.x, a.x);
        assert_relative_eq!(b.y, a.y);
        assert_relative_eq!(b.z, a.z);
    }

    #[test]
    fn adding_point_vec() {
        let p: Point<spaces::World> = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);
        assert_relative_eq!(p + v, Point::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn subbing_point_vec() {
        let p: Point<spaces::World> = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);
        assert_relative_eq!(p - v, Point::new(5.0, -5.0, 4.0));
    }

    #[test]
    fn subbing_two_points() {
        let p1: Point<spaces::World> = Point::new(3.0, -2.0, 5.0);
        let p2: Point<spaces::World> = Point::new(1.0, 1.0, 1.0);
        assert_relative_eq!(p1 - p2, Vector::new(2.0, -3.0, 4.0));
    }

    /*
        /// Subtracting two points
        #[test]
        fn subtracting_two_points() {
            let p1: Tup<spaces::World> = Tup::point(3, 2, 1);
            let p2 = Tup::point(5, 6, 7);
            assert_relative_eq!(p1 - p2, Tup::vector(-2, -4, -6));
        }

        /// Subtracting a vector from a point
        #[test]
        fn subtracting_vec_from_point() {
            let p: Tup<spaces::World> = Tup::point(3, 2, 1);
            let v = Tup::vector(5, 6, 7);
            assert_relative_eq!(p - v, Tup::point(-2, -4, -6));
        }

        /// Subtracting two vectors
        #[test]
        fn subtracting_vectors() {
            let v1: Tup<spaces::World> = Tup::vector(3, 2, 1);
            let v2 = Tup::vector(5, 6, 7);
            assert_relative_eq!(v1 - v2, Tup::vector(-2, -4, -6));
        }

        /// Subtracting a vector from the zero vector
        #[test]
        fn subtracting_vector_from_zero() {
            let zero: Tup<spaces::World> = Tup::vector(0, 0, 0);
            let v2 = Tup::vector(1, -2, 3);
            assert_relative_eq!(zero - v2, Tup::vector(-1, 2, -3));
        }

        /// Negating a tuple
        #[test]
        fn negating_tuple() {
            let a: Tup<spaces::World> = Tup::new(1.0, -2.0, 3.0, -4.0);
            assert_relative_eq!(-a, Tup::new(-1.0, 2.0, -3.0, 4.0));
        }

        /// Multiplying a tuple by a scalar
        #[test]
        fn mult_tup_by_scalar() {
            let a: Tup<spaces::World> = Tup::new(1.0, -2.0, 3.0, -4.0);
            assert_relative_eq!(a * 3.5, Tup::new(3.5, -7.0, 10.5, -14.0,));
        }

        /// Multiplying a tuple by a fraction
        #[test]
        fn mult_tup_by_fraction() {
            let a: Tup<spaces::World> = Tup::new(1.0, -2.0, 3.0, -4.0);
            assert_relative_eq!(a * 0.5, Tup::new(0.5, -1.0, 1.5, -2.0,));
        }

        /// Dividing a tuple by a scalar
        #[test]
        fn div_tup_by_scalar() {
            let a: Tup<spaces::World> = Tup::new(1.0, -2.0, 3.0, -4.0);
            assert_relative_eq!(a / 2.0, Tup::new(0.5, -1.0, 1.5, -2.0,));
        }

        /// Computing the magnitude of a vector(1, 0, 0)
        #[test]
        fn vec_magnitude_100() {
            let v: Tup<spaces::World> = Tup::vector(1, 0, 0);
            assert_relative_eq!(v.magnitude(), 1.0);
        }

        /// Computing the magnitude of a vector(0, 1, 0)
        #[test]
        fn vec_magnitude_010() {
            let v: Tup<spaces::World> = Tup::vector(0, 1, 0);
            assert_relative_eq!(v.magnitude(), 1.0);
        }

        /// Computing the magnitude of a vector(0, 0, 1)
        #[test]
        fn vec_magnitude_001() {
            let v: Tup<spaces::World> = Tup::vector(0, 0, 1);
            assert_relative_eq!(v.magnitude(), 1.0);
        }

        /// Computing the magnitude of a vector(1, 2, 3)
        #[test]
        fn vec_magnitude_123() {
            let v: Tup<spaces::World> = Tup::vector(1, 2, 3);
            assert_relative_eq!(v.magnitude(), 14f64.sqrt());
        }

        /// Computing the magnitude of a vector(-1, -2, -3)
        #[test]
        fn vec_magnitude_neg_123() {
            let v: Tup<spaces::World> = Tup::vector(-1, -2, -3);
            assert_relative_eq!(v.magnitude(), 14f64.sqrt());
        }

        /// Normalizing vector(4, 0, 0) gives (1, 0, 0)
        #[test]
        fn vec_normalize_400() {
            let v: Tup<spaces::World> = Tup::vector(4, 0, 0);
            assert_relative_eq!(v.normalize(), Tup::vector(1, 0, 0));
        }

        /// Normalizing vector(1, 2, 3)
        #[test]
        fn vec_normalize_123() {
            let v: Tup<spaces::World> = Tup::vector(1, 2, 3);
            assert_relative_eq!(
                v.normalize(),
                Tup::vector(1.0 / 14f64.sqrt(), 2.0 / 14f64.sqrt(), 3.0 / 14f64.sqrt())
            );
        }

        /// Magnitude of a normalized vector
        #[test]
        fn vec_normalized_magnitude() {
            let v: Tup<spaces::World> = Tup::vector(1, 2, 3);
            assert_relative_eq!(v.normalize().magnitude(), 1.0);
        }

        /// The cross product of two tuples
        #[test]
        fn cross_product() {
            let a: Tup<spaces::World> = Tup::vector(1, 2, 3);
            let b = Tup::vector(2, 3, 4);
            assert_relative_eq!(a.cross(b), Tup::vector(-1, 2, -1));
            assert_relative_eq!(b.cross(a), Tup::vector(1, -2, 1));
        }

        /// The dot product of two tuples
        #[test]
        fn dot_product() {
            let a: Tup<spaces::World> = Tup::vector(1, 2, 3);
            let b = Tup::vector(2, 3, 4);
            assert_relative_eq!(a.dot(b), 20.0);
        }

        #[test]
        fn reflect_approaching_at_45() {
            let v: Tup<spaces::World> = Tup::vector(1, -1, 0);
            let n = Tup::vector(0, 1, 0);
            assert_relative_eq!(v.reflect(n), Tup::vector(1, 1, 0));
        }

        #[test]
        fn reflect_slanted() {
            let v: Tup<spaces::World> = Tup::vector(0, -1, 0);
            let n = Tup::vector(2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0, 0);
            assert_relative_eq!(v.reflect(n), Tup::vector(1, 0, 0));
        }
    */
}
