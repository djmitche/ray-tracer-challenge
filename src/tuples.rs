#![allow(dead_code)]
#![allow(unused_imports)]
use approx::{relative_eq, AbsDiffEq, RelativeEq};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Tup {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

/// Tup represents a tuple.
impl Tup {
    /// Create a new tuple.
    pub fn new<X: Into<f64>, Y: Into<f64>, Z: Into<f64>, W: Into<f64>>(
        x: X,
        y: Y,
        z: Z,
        w: W,
    ) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: w.into(),
        }
    }
    /// Create a new tuple as a point (with w coordinate equal to 1)
    pub fn point<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(x: X, y: Y, z: Z) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: 1.0,
        }
    }

    /// Create a new tuple as a vector (with w coordinate equal to 0)
    pub fn vector<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(x: X, y: Y, z: Z) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: 0.0,
        }
    }

    /// Determine if this is a point (w coordinate equal to 1)
    pub fn is_point(&self) -> bool {
        relative_eq!(self.w, 1.0)
    }

    /// Convert a vector to a point.
    pub fn as_point(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
            w: 1.0,
        }
    }

    /// Determine if this is a vector (w coordinate equal to 0)
    pub fn is_vector(&self) -> bool {
        relative_eq!(self.w, 0.0)
    }

    /// Convert a point to a vector
    pub fn as_vector(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
            w: 0.0,
        }
    }

    /// Determine the magnitude of this tuple
    pub fn magnitude(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)).sqrt()
    }

    /// Return a scaled vector with magnitude 1.0.  This will fail for a zero tuple.
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Tup {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    /// Compute the dot product of two tuples
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    /// Compute the cross product of two tuples
    pub fn cross(self, other: Self) -> Self {
        Tup::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl std::ops::Index<usize> for Tup {
    type Output = f64;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => unreachable!(),
        }
    }
}

impl std::ops::IndexMut<usize> for Tup {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => unreachable!(),
        }
    }
}

impl AbsDiffEq for Tup {
    type Epsilon = <f64 as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> <f64 as AbsDiffEq>::Epsilon {
        <f64 as AbsDiffEq>::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: <f64 as AbsDiffEq>::Epsilon) -> bool {
        <f64 as AbsDiffEq>::abs_diff_eq(&self.x, &other.x, epsilon)
            && <f64 as AbsDiffEq>::abs_diff_eq(&self.y, &other.y, epsilon)
            && <f64 as AbsDiffEq>::abs_diff_eq(&self.z, &other.z, epsilon)
            && <f64 as AbsDiffEq>::abs_diff_eq(&self.w, &other.w, epsilon)
    }
}

impl RelativeEq for Tup {
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
            && <f64 as RelativeEq>::relative_eq(&self.w, &other.w, epsilon, max_relative)
    }
}

impl std::ops::Add for Tup {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl std::ops::Sub for Tup {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl std::ops::Neg for Tup {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl std::ops::Mul<f64> for Tup {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl std::ops::Div<f64> for Tup {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;

    /// A tuple with w=1.0 is a point
    #[test]
    fn tuple() {
        let a = Tup::new(4.3, -4.2, 3.1, 2.0);
        assert_relative_eq!(a.x, 4.3);
        assert_relative_eq!(a[0], 4.3);
        assert_relative_eq!(a.y, -4.2);
        assert_relative_eq!(a[1], -4.2);
        assert_relative_eq!(a.z, 3.1);
        assert_relative_eq!(a[2], 3.1);
        assert_relative_eq!(a.w, 2.0);
        assert_relative_eq!(a[3], 2.0);
        assert!(!a.is_point());
        assert!(!a.is_vector());
    }

    /// A tuple with w=1.0 is a point
    #[test]
    fn tuple_is_point() {
        let a = Tup {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };
        assert_relative_eq!(a.x, 4.3);
        assert_relative_eq!(a.y, -4.2);
        assert_relative_eq!(a.z, 3.1);
        assert_relative_eq!(a.w, 1.0);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    /// A tuple with w=0.0 is a vector
    #[test]
    fn tuple_is_vector() {
        let a = Tup {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };
        assert_relative_eq!(a.x, 4.3);
        assert_relative_eq!(a.y, -4.2);
        assert_relative_eq!(a.z, 3.1);
        assert_relative_eq!(a.w, 0.0);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    /// Tuple::point creates tuples with w=1.0
    #[test]
    fn tuple_point() {
        let p = Tup::point(4, -4, 3);
        assert_relative_eq!(
            p,
            Tup {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 1.0
            }
        );
    }

    /// Tuple::vector creates tuples with w=0.0
    #[test]
    fn tuple_vector() {
        let p = Tup::vector(4, -4, 3);
        assert_relative_eq!(
            p,
            Tup {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 0.0
            }
        );
    }

    /// Adding two tuples
    #[test]
    fn adding_two_tuples() {
        let a1 = Tup {
            x: 3.0,
            y: -2.0,
            z: 5.0,
            w: 1.0,
        };
        let a2 = Tup {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 0.0,
        };
        assert_relative_eq!(
            a1 + a2,
            Tup {
                x: 1.0,
                y: 1.0,
                z: 6.0,
                w: 1.0
            }
        );
    }

    /// Subtracting two points
    #[test]
    fn subtracting_two_points() {
        let p1 = Tup::point(3, 2, 1);
        let p2 = Tup::point(5, 6, 7);
        assert_relative_eq!(p1 - p2, Tup::vector(-2, -4, -6));
    }

    /// Subtracting a vector from a point
    #[test]
    fn subtracting_vec_from_point() {
        let p = Tup::point(3, 2, 1);
        let v = Tup::vector(5, 6, 7);
        assert_relative_eq!(p - v, Tup::point(-2, -4, -6));
    }

    /// Subtracting two vectors
    #[test]
    fn subtracting_vectors() {
        let v1 = Tup::vector(3, 2, 1);
        let v2 = Tup::vector(5, 6, 7);
        assert_relative_eq!(v1 - v2, Tup::vector(-2, -4, -6));
    }

    /// Subtracting a vector from the zero vector
    #[test]
    fn subtracting_vector_from_zero() {
        let zero = Tup::vector(0, 0, 0);
        let v2 = Tup::vector(1, -2, 3);
        assert_relative_eq!(zero - v2, Tup::vector(-1, 2, -3));
    }

    /// Negating a tuple
    #[test]
    fn negating_tuple() {
        let a = Tup {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        assert_relative_eq!(
            -a,
            Tup {
                x: -1.0,
                y: 2.0,
                z: -3.0,
                w: 4.0
            }
        );
    }

    /// Multiplying a tuple by a scalar
    #[test]
    fn mult_tup_by_scalar() {
        let a = Tup {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        assert_relative_eq!(
            a * 3.5,
            Tup {
                x: 3.5,
                y: -7.0,
                z: 10.5,
                w: -14.0,
            }
        );
    }

    /// Multiplying a tuple by a fraction
    #[test]
    fn mult_tup_by_fraction() {
        let a = Tup {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        assert_relative_eq!(
            a * 0.5,
            Tup {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0,
            }
        );
    }

    /// Dividing a tuple by a scalar
    #[test]
    fn div_tup_by_scalar() {
        let a = Tup {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        assert_relative_eq!(
            a / 2.0,
            Tup {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0,
            }
        );
    }

    /// Computing the magnitude of a vector(1, 0, 0)
    #[test]
    fn vec_magnitude_100() {
        let v = Tup::vector(1, 0, 0);
        assert_relative_eq!(v.magnitude(), 1.0);
    }

    /// Computing the magnitude of a vector(0, 1, 0)
    #[test]
    fn vec_magnitude_010() {
        let v = Tup::vector(0, 1, 0);
        assert_relative_eq!(v.magnitude(), 1.0);
    }

    /// Computing the magnitude of a vector(0, 0, 1)
    #[test]
    fn vec_magnitude_001() {
        let v = Tup::vector(0, 0, 1);
        assert_relative_eq!(v.magnitude(), 1.0);
    }

    /// Computing the magnitude of a vector(1, 2, 3)
    #[test]
    fn vec_magnitude_123() {
        let v = Tup::vector(1, 2, 3);
        assert_relative_eq!(v.magnitude(), 14f64.sqrt());
    }

    /// Computing the magnitude of a vector(-1, -2, -3)
    #[test]
    fn vec_magnitude_neg_123() {
        let v = Tup::vector(-1, -2, -3);
        assert_relative_eq!(v.magnitude(), 14f64.sqrt());
    }

    /// Normalizing vector(4, 0, 0) gives (1, 0, 0)
    #[test]
    fn vec_normalize_400() {
        let v = Tup::vector(4, 0, 0);
        assert_relative_eq!(v.normalize(), Tup::vector(1, 0, 0));
    }

    /// Normalizing vector(1, 2, 3)
    #[test]
    fn vec_normalize_123() {
        let v = Tup::vector(1, 2, 3);
        assert_relative_eq!(
            v.normalize(),
            Tup::vector(1.0 / 14f64.sqrt(), 2.0 / 14f64.sqrt(), 3.0 / 14f64.sqrt())
        );
    }

    /// Magnitude of a normalized vector
    #[test]
    fn vec_normalized_magnitude() {
        let v = Tup::vector(1, 2, 3);
        assert_relative_eq!(v.normalize().magnitude(), 1.0);
    }

    /// The cross product of two tuples
    #[test]
    fn cross_product() {
        let a = Tup::vector(1, 2, 3);
        let b = Tup::vector(2, 3, 4);
        assert_relative_eq!(a.cross(b), Tup::vector(-1, 2, -1));
        assert_relative_eq!(b.cross(a), Tup::vector(1, -2, 1));
    }

    /// The dot product of two tuples
    #[test]
    fn dot_product() {
        let a = Tup::vector(1, 2, 3);
        let b = Tup::vector(2, 3, 4);
        assert_relative_eq!(a.dot(b), 20.0);
    }
}
