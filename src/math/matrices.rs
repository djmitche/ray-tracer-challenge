use crate::{Point, Space, Vector};
use approx::{AbsDiffEq, RelativeEq};
use std::marker::PhantomData;

/// An NxN matrix giving a transformation from S1 to S2.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat<const N: usize, S1: Space, S2: Space>([[f64; N]; N], PhantomData<(S1, S2)>);

#[macro_export]
macro_rules! mat2 {
    [$a0:expr , $a1:expr ;
    $b0:expr , $b1:expr ;
    ] => {
        $crate::Mat::new([
            [$a0.into(), $a1.into()],
            [$b0.into(), $b1.into()],
        ])
    }
}

#[macro_export]
macro_rules! mat3 {
    [$a0:expr , $a1:expr , $a2:expr ;
    $b0:expr , $b1:expr , $b2:expr ;
    $c0:expr , $c1:expr , $c2:expr ;
    ] => {
        $crate::Mat::new([
            [$a0.into(), $a1.into(), $a2.into()],
            [$b0.into(), $b1.into(), $b2.into()],
            [$c0.into(), $c1.into(), $c2.into()],
        ])
    }
}

#[macro_export]
macro_rules! mat4 {
    [$a0:expr , $a1:expr , $a2:expr , $a3:expr ;
    $b0:expr , $b1:expr , $b2:expr , $b3:expr ;
    $c0:expr , $c1:expr , $c2:expr , $c3:expr ;
    $d0:expr , $d1:expr , $d2:expr , $d3:expr ;
    ] => {
        $crate::Mat::new([
            [$a0.into(), $a1.into(), $a2.into(), $a3.into()],
            [$b0.into(), $b1.into(), $b2.into(), $b3.into()],
            [$c0.into(), $c1.into(), $c2.into(), $c3.into()],
            [$d0.into(), $d1.into(), $d2.into(), $d3.into()],
        ])
    }
}

impl<const N: usize, S1: Space, S2: Space> Mat<N, S1, S2> {
    /// Create a new matrix with the given values (an array of rows)
    pub fn new(vals: [[f64; N]; N]) -> Self {
        Self(vals, PhantomData)
    }

    pub fn identity() -> Self {
        let mut res = Self::default();
        for i in 0..N {
            res[(i, i)] = 1.0;
        }
        res
    }

    pub fn transpose(&self) -> Mat<N, S2, S1> {
        let mut res = Mat::default();
        for i in 0..N {
            for j in 0..N {
                res[(i, j)] = self[(j, i)]
            }
        }
        res
    }
}

impl<S1: Space, S2: Space> Mat<2, S1, S2> {
    pub fn determinant(&self) -> f64 {
        self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
    }
}

impl<S1: Space, S2: Space> Mat<3, S1, S2> {
    pub fn submatrix(self, r: usize, c: usize) -> Mat<2, S1, S2> {
        let mut res: Mat<2, S1, S2> = Default::default();
        let mut ii = 0;
        for i in 0..3 {
            if i == r {
                continue;
            }
            let mut jj = 0;
            for j in 0..3 {
                if j == c {
                    continue;
                }
                res[(ii, jj)] = self[(i, j)];
                jj += 1;
            }
            ii += 1;
        }
        res
    }

    pub fn minor(self, r: usize, c: usize) -> f64 {
        self.submatrix(r, c).determinant()
    }

    pub fn cofactor(self, r: usize, c: usize) -> f64 {
        let m = self.minor(r, c);
        if (r + c) & 1 == 0 {
            m
        } else {
            -m
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut res = 0.0;
        for i in 0..3 {
            res += self[(0, i)] * self.cofactor(0, i);
        }
        res
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }
}

impl<S1: Space, S2: Space> Mat<4, S1, S2> {
    pub fn submatrix(self, r: usize, c: usize) -> Mat<3, S1, S2> {
        let mut res: Mat<3, S1, S2> = Default::default();
        let mut ii = 0;
        for i in 0..4 {
            if i == r {
                continue;
            }
            let mut jj = 0;
            for j in 0..4 {
                if j == c {
                    continue;
                }
                res[(ii, jj)] = self[(i, j)];
                jj += 1;
            }
            ii += 1;
        }
        res
    }

    pub fn minor(self, r: usize, c: usize) -> f64 {
        self.submatrix(r, c).determinant()
    }

    pub fn cofactor(self, r: usize, c: usize) -> f64 {
        let m = self.minor(r, c);
        if (r + c) & 1 == 0 {
            m
        } else {
            -m
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut res = 0.0;
        for i in 0..4 {
            res += self[(0, i)] * self.cofactor(0, i);
        }
        res
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Mat<4, S2, S1> {
        let mut res: Mat<4, S2, S1> = Default::default();
        let det = self.determinant();
        for r in 0..4 {
            for c in 0..4 {
                res[(c, r)] = self.cofactor(r, c) / det;
            }
        }
        res
    }

    pub fn translate<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(&self, x: X, y: Y, z: Z) -> Self {
        return mat4![
            1, 0, 0, x;
            0, 1, 0, y;
            0, 0, 1, z;
            0, 0, 0, 1;
        ] * *self;
    }

    pub fn scale<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(&self, x: X, y: Y, z: Z) -> Self {
        return mat4![
            x, 0, 0, 0;
            0, y, 0, 0;
            0, 0, z, 0;
            0, 0, 0, 1;
        ] * *self;
    }

    pub fn rotate_x<R: Into<f64>>(&self, r: R) -> Self {
        let r = r.into();
        let c = r.cos();
        let s = r.sin();
        return mat4![
            1, 0,  0, 0;
            0, c, -s, 0;
            0, s,  c, 0;
            0, 0,  0, 1;
        ] * *self;
    }

    pub fn rotate_y<R: Into<f64>>(&self, r: R) -> Self {
        let r = r.into();
        let c = r.cos();
        let s = r.sin();
        return mat4![
            c,  0, s, 0;
            0,  1, 0, 0;
            -s, 0, c, 0;
            0,  0, 0, 1;
        ] * *self;
    }

    pub fn rotate_z<R: Into<f64>>(&self, r: R) -> Self {
        let r = r.into();
        let c = r.cos();
        let s = r.sin();
        return mat4![
            c, -s, 0, 0;
            s,  c, 0, 0;
            0,  0, 1, 0;
            0,  0, 0, 1;
        ] * *self;
    }

    pub fn shear<
        XY: Into<f64>,
        XZ: Into<f64>,
        YX: Into<f64>,
        YZ: Into<f64>,
        ZX: Into<f64>,
        ZY: Into<f64>,
    >(
        &self,
        xy: XY,
        xz: XZ,
        yx: YX,
        yz: YZ,
        zx: ZX,
        zy: ZY,
    ) -> Self {
        return mat4![
            1, xy,  xz, 0;
            yx, 1,  yz, 0;
            zx, zy, 1,  0;
            0,  0,  0,  1;
        ] * *self;
    }
}

impl<const N: usize, S1: Space, S2: Space> Default for Mat<N, S1, S2> {
    fn default() -> Self {
        Self([[0f64; N]; N], PhantomData)
    }
}

impl<const N: usize, S1: Space, S2: Space> std::ops::Index<(usize, usize)> for Mat<N, S1, S2> {
    type Output = f64;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.0[idx.0][idx.1]
    }
}

impl<const N: usize, S1: Space, S2: Space> std::ops::IndexMut<(usize, usize)> for Mat<N, S1, S2> {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        &mut self.0[idx.0][idx.1]
    }
}

impl<const N: usize, S1: Space, S2: Space> AbsDiffEq for Mat<N, S1, S2> {
    type Epsilon = <f64 as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> <f64 as AbsDiffEq>::Epsilon {
        <f64 as AbsDiffEq>::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: <f64 as AbsDiffEq>::Epsilon) -> bool {
        for i in 0..N {
            for j in 0..N {
                if !<f64 as AbsDiffEq>::abs_diff_eq(&self[(i, j)], &other[(i, j)], epsilon) {
                    return false;
                }
            }
        }
        true
    }
}

impl<const N: usize, S1: Space, S2: Space> RelativeEq for Mat<N, S1, S2> {
    fn default_max_relative() -> <f64 as AbsDiffEq>::Epsilon {
        <f64 as RelativeEq>::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: <f64 as AbsDiffEq>::Epsilon,
        max_relative: <f64 as AbsDiffEq>::Epsilon,
    ) -> bool {
        for i in 0..N {
            for j in 0..N {
                if !<f64 as RelativeEq>::relative_eq(
                    &self[(i, j)],
                    &other[(i, j)],
                    epsilon,
                    max_relative,
                ) {
                    return false;
                }
            }
        }
        true
    }
}

impl<const N: usize, S1: Space, S2: Space, S3: Space> std::ops::Mul<Mat<N, S1, S2>>
    for Mat<N, S2, S3>
{
    type Output = Mat<N, S1, S3>;
    fn mul(self, other: Mat<N, S1, S2>) -> Mat<N, S1, S3> {
        let mut res = Self::Output::default();
        for i in 0..N {
            for j in 0..N {
                let mut v = 0.0;
                for k in 0..N {
                    v += self[(i, k)] * other[(k, j)];
                }
                res[(i, j)] = v;
            }
        }
        res
    }
}

impl<S1: Space, S2: Space> std::ops::Mul<Vector<S1>> for Mat<4, S1, S2> {
    type Output = Vector<S2>;
    fn mul(self, other: Vector<S1>) -> Vector<S2> {
        Vector::new(
            // implicit 4th element of `other` is 0.0
            other.x * self[(0, 0)] + other.y * self[(0, 1)] + other.z * self[(0, 2)],
            other.x * self[(1, 0)] + other.y * self[(1, 1)] + other.z * self[(1, 2)],
            other.x * self[(2, 0)] + other.y * self[(2, 1)] + other.z * self[(2, 2)],
        )
    }
}

impl<S1: Space, S2: Space> std::ops::Mul<Point<S1>> for Mat<4, S1, S2> {
    type Output = Point<S2>;
    fn mul(self, other: Point<S1>) -> Point<S2> {
        Point::new(
            // implicit 4th element of `other` is 1.0
            other.x * self[(0, 0)] + other.y * self[(0, 1)] + other.z * self[(0, 2)] + self[(0, 3)],
            other.x * self[(1, 0)] + other.y * self[(1, 1)] + other.z * self[(1, 2)] + self[(1, 3)],
            other.x * self[(2, 0)] + other.y * self[(2, 1)] + other.z * self[(2, 2)] + self[(2, 3)],
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::spaces;
    use approx::*;
    use std::f64::consts::PI;

    /// Constructing and inspecting a 4x4 matrix
    #[test]
    fn test_4x4() {
        let mat: Mat<4, spaces::World, spaces::World> = mat4![
            1, 2, 3, 4;
            5.5, 6.5, 7.5, 8.5;
            9, 10, 11, 12;
            13.5, 14.5, 15.5, 16.5;
        ];

        assert_relative_eq!(mat[(0, 0)], 1.0);
        assert_relative_eq!(mat[(0, 3)], 4.0);
        assert_relative_eq!(mat[(1, 0)], 5.5);
        assert_relative_eq!(mat[(1, 2)], 7.5);
        assert_relative_eq!(mat[(2, 2)], 11.0);
        assert_relative_eq!(mat[(3, 0)], 13.5);
        assert_relative_eq!(mat[(3, 2)], 15.5);
    }

    /// Constructing and inspecting a 2x2 matrix
    #[test]
    fn test_2x2() {
        let mat: Mat<2, spaces::World, spaces::World> = mat2![
            -3, 5;
            1, -2;
        ];

        assert_relative_eq!(mat[(0, 0)], -3.0);
        assert_relative_eq!(mat[(0, 1)], 5.0);
        assert_relative_eq!(mat[(1, 0)], 1.0);
        assert_relative_eq!(mat[(1, 1)], -2.0);
    }

    /// Constructing and inspecting a 3x3 matrix
    #[test]
    fn test_3x3() {
        let mat: Mat<3, spaces::World, spaces::World> = mat3![
            -3, 5, 0;
            1, -2, -7;
            0, 1, 1;
        ];

        assert_relative_eq!(mat[(0, 0)], -3.0);
        assert_relative_eq!(mat[(1, 1)], -2.0);
        assert_relative_eq!(mat[(2, 2)], 1.0);
    }

    /// Matrix equality with different matrices
    #[test]
    fn equality_ne() {
        let a: Mat<4, spaces::World, spaces::World> = mat4![
            1, 2, 3, 4;
            5, 6, 7, 8;
            13, 14, 15, 16;
            17, 18, 19, 20;
        ];
        let b: Mat<4, spaces::World, spaces::World> = mat4![
            5, 6, 7, 8;
            1, 2, 3, 4;
            17, 18, 19, 20;
            13, 14, 15, 16;
        ];
        assert_relative_ne!(a, b);
    }

    /// Matrix equality with identical matrices
    #[test]
    fn equality_eq() {
        let a: Mat<4, spaces::World, spaces::World> = mat4![
            1, 2, 3, 4;
            5, 6, 7, 8;
            13, 14, 15, 16;
            17, 18, 19, 20;
        ];
        let b: Mat<4, spaces::World, spaces::World> = mat4![
            1, 2, 3, 4;
            5, 6, 7, 8;
            13, 14, 15, 16;
            17, 18, 19, 20;
        ];
        assert_relative_eq!(a, b);
    }

    /// Multiplying two matrices
    #[test]
    fn multiply_mat() {
        let a: Mat<4, spaces::World, spaces::Object> = mat4![
            1, 2, 3, 4;
            5, 6, 7, 8;
            9, 8, 7, 6;
            5, 4, 3, 2;
        ];
        let b: Mat<4, spaces::Camera, spaces::World> = mat4![
            -2, 1, 2, 3;
            3, 2, 1, -1;
            4, 3, 6, 5;
            1, 2, 7, 8;
        ];
        let res: Mat<4, spaces::Camera, spaces::Object> = mat4![
            20, 22, 50, 48;
            44, 54, 114, 108;
            40, 58, 110, 102;
            16, 26, 46, 42;
        ];
        assert_relative_eq!(a * b, res);
    }

    /// A matrix multiplied by a vector.
    #[test]
    fn multiply_vector() {
        let a: Mat<4, spaces::Camera, spaces::World> = mat4![
            1, 2, 3, 4;
            2, 4, 4, 2;
            8, 6, 4, 1;
            0, 0, 0, 1;
        ];
        let b: Vector<spaces::Camera> = Vector::new(1, 2, 3);
        let res: Vector<spaces::World> = Vector::new(14, 22, 32);
        assert_relative_eq!(a * b, res);
    }

    /// A matrix multiplied by a point.
    #[test]
    fn multiply_point() {
        let a: Mat<4, spaces::Camera, spaces::World> = mat4![
            1, 2, 3, 4;
            2, 4, 4, 2;
            8, 6, 4, 1;
            0, 0, 0, 1;
        ];
        let b: Point<spaces::Camera> = Point::new(1, 2, 3);
        let res: Point<spaces::World> = Point::new(18, 24, 33);
        assert_relative_eq!(a * b, res);
    }

    /// Multiplying by the identity matrix
    #[test]
    fn multiply_mat_identity() {
        let a: Mat<4, spaces::World, spaces::World> = mat4![
            1, 2, 3, 4;
            5, 6, 7, 8;
            9, 8, 7, 6;
            5, 4, 3, 2;
        ];
        assert_relative_eq!(a * Mat::identity(), a);
        assert_relative_eq!(Mat::identity() * a, a);
    }

    /// Transposing a 4x4 matrix
    #[test]
    fn mat_transpose_4x4() {
        let a: Mat<4, spaces::World, spaces::World> = mat4![
            1, 2, 3, 4;
            5, 6, 7, 8;
            9, 8, 7, 6;
            5, 4, 3, 2;
        ];
        let b: Mat<4, spaces::World, spaces::World> = mat4![
            1, 5, 9, 5;
            2, 6, 8, 4;
            3, 7, 7, 3;
            4, 8, 6, 2;
        ];
        assert_relative_eq!(a.transpose(), b);
        assert_relative_eq!(b.transpose(), a);
    }

    /// Transposing a 2x2 matrix
    #[test]
    fn mat_transpose_2x2() {
        let a: Mat<2, spaces::World, spaces::World> = mat2![
            1, 2;
            5, 6;
        ];
        let b: Mat<2, spaces::World, spaces::World> = mat2![
            1, 5;
            2, 6;
        ];
        assert_relative_eq!(a.transpose(), b);
        assert_relative_eq!(b.transpose(), a);
    }

    /// Transposing the identity matrix
    #[test]
    fn mat_transpose_ident() {
        assert_relative_eq!(
            Mat::<1, spaces::World, spaces::World>::identity().transpose(),
            Mat::identity()
        );
        assert_relative_eq!(
            Mat::<2, spaces::World, spaces::World>::identity().transpose(),
            Mat::identity()
        );
        assert_relative_eq!(
            Mat::<3, spaces::World, spaces::World>::identity().transpose(),
            Mat::identity()
        );
        assert_relative_eq!(
            Mat::<4, spaces::World, spaces::World>::identity().transpose(),
            Mat::identity()
        );
    }

    /// Determinant of a 2x2 matrix
    #[test]
    fn determinant_2x2() {
        let a: Mat<2, spaces::World, spaces::World> = mat2![
            1, 5;
            -3, 2;
        ];
        assert_relative_eq!(a.determinant(), 17.0);
    }

    /// Submatrix of a 3x3 matrix
    #[test]
    fn submat_3x3() {
        let a: Mat<3, spaces::World, spaces::World> = mat3![
            1, 5, 0;
            -3, 2, 7;
            0, 6, -3;
        ];
        let res = mat2![
            -3, 2;
            0, 6;
        ];
        assert_relative_eq!(a.submatrix(0, 2), res);
    }

    /// Submatrix of a 4x4 matrix
    #[test]
    fn submat_4x4() {
        let a: Mat<4, spaces::World, spaces::World> = mat4![
            1, 2, 3, 4;
            5, 6, 7, 8;
            9, 8, 7, 6;
            5, 4, 3, 2;
        ];
        let res = mat3![
            1, 3, 4;
            5, 7, 8;
            5, 3, 2;
        ];
        assert_relative_eq!(a.submatrix(2, 1), res);
    }

    /// Calculating a minor of a 3x3 matrix
    #[test]
    fn minor_3x3() {
        let a: Mat<3, spaces::World, spaces::World> = mat3![
            3, 5, 0;
            2, -1, -7;
            6, -1, 5;
        ];
        assert_relative_eq!(a.minor(1, 0), 25.0);
    }

    /// Calculating a cofactor of a 3x3 matrix
    #[test]
    fn cofactor_3x3() {
        let a: Mat<3, spaces::World, spaces::World> = mat3![
            3, 5, 0;
            2, -1, -7;
            6, -1, 5;
        ];
        assert_relative_eq!(a.minor(0, 0), -12.0);
        assert_relative_eq!(a.cofactor(0, 0), -12.0);
        assert_relative_eq!(a.minor(1, 0), 25.0);
        assert_relative_eq!(a.cofactor(1, 0), -25.0);
    }

    /// Calculating the determinant of a 3x3 matrix
    #[test]
    fn det_3x3() {
        let a: Mat<3, spaces::World, spaces::World> = mat3![
            1, 2, 6;
            -5, 8, -4;
            2, 6, 4;
        ];
        assert_relative_eq!(a.cofactor(0, 0), 56.0);
        assert_relative_eq!(a.cofactor(0, 1), 12.0);
        assert_relative_eq!(a.cofactor(0, 2), -46.0);
        assert_relative_eq!(a.determinant(), -196.0);
    }

    /// Calculating the determinant of a 4x4 matrix
    #[test]
    fn det_4x4() {
        let a: Mat<4, spaces::World, spaces::World> = mat4![
            -2, -8, 3, 5;
            -3, 1, 7, 3;
            1, 2, -9, 6;
            -6, 7, 7, -9;
        ];
        assert_relative_eq!(a.determinant(), -4071.0);
    }

    /// Testing an invertible matrix for invertibility
    #[test]
    fn invertibility_yes() {
        let a: Mat<4, spaces::World, spaces::World> = mat4![
            6, 4, 4, 4;
            5, 5, 7, 6;
            4, -9, 3, -7;
            9, 1, 7, -6;
        ];
        assert_relative_eq!(a.determinant(), -2120.0);
        assert!(a.is_invertible());
    }

    /// Testing a non-invertible matrix for invertibility
    #[test]
    fn invertibility_no() {
        let a: Mat<4, spaces::World, spaces::World> = mat4![
            -4, 2, -2, -3;
            9, 6, 2, 6;
            0, -5, 1, -5;
            0, 0, 0, 0;
        ];
        assert_relative_eq!(a.determinant(), 0.0);
        assert!(!a.is_invertible());
    }

    /// Calcuating the inverse of a matrix
    #[test]
    fn invert() {
        let a: Mat<4, spaces::World, spaces::World> = mat4![
            -5, 2, 6, -8;
            1, -5, 1, 8;
            7, 7, -6, -7;
            1, -3, 7, 4;
        ];
        let b = a.inverse();
        assert_relative_eq!(a.determinant(), 532.0);
        assert!(a.is_invertible());
        assert_relative_eq!(a.cofactor(2, 3), -160.0);
        assert_relative_eq!(b[(3, 2)], -160.0 / 532.0);
        assert_relative_eq!(a.cofactor(3, 2), 105.0);
        assert_relative_eq!(b[(2, 3)], 105.0 / 532.0);

        let inverted = mat4![
            0.21805, 0.45113, 0.24060, -0.04511;
            -0.80827, -1.45677, -0.44361, 0.52068;
            -0.07895, -0.22368, -0.05263, 0.19737;
            -0.52256, -0.81391, -0.30075, 0.30639;
        ];
        assert!(Relative {
            epsilon: 0.00001,
            max_relative: 0.00001,
        }
        .eq(&b, &inverted));
    }

    /// Calcuating the inverse of a second matrix
    #[test]
    fn invert_2() {
        let a: Mat<4, spaces::World, spaces::World> = mat4![
            8, -5, 9, 2;
            7, 5, 6, 1;
            -6, 0, 9, 6;
            -3, 0, -9, -4;
        ];
        let b = a.inverse();
        let inverted = mat4![
            -0.15385, -0.15385, -0.28205, -0.53846;
            -0.07692, 0.12308, 0.02564, 0.03077;
            0.35897, 0.35897, 0.43590, 0.92308;
            -0.69231, -0.69231, -0.76923, -1.92308;
        ];
        assert!(Relative {
            epsilon: 0.00001,
            max_relative: 0.00001,
        }
        .eq(&b, &inverted));
    }

    /// Multiplying a matrix by its inverse
    #[test]
    fn multiply_by_inverse() {
        let a: Mat<4, spaces::World, spaces::World> = mat4![
            8, -5, 9, 2;
            7, 5, 6, 1;
            -6, 0, 9, 6;
            -3, 0, -9, -4;
        ];
        let b: Mat<4, spaces::World, spaces::World> = mat4![
            -5, 2, 6, -8;
            1, -5, 1, 8;
            7, 7, -6, -7;
            1, -3, 7, 4;
        ];
        let c = a * b;
        assert!(Relative {
            epsilon: 0.00001,
            max_relative: 0.00001,
        }
        .eq(&(c * b.inverse()), &a));
    }

    #[test]
    fn mult_by_translation() {
        let transform: Mat<4, spaces::World, spaces::World> = Mat::identity().translate(5, -3, 2);
        let p = Point::new(-3, 4, 5);
        assert_relative_eq!(transform * p, Point::new(2, 1, 7));
    }

    #[test]
    fn mult_by_inv_translation() {
        let transform: Mat<4, spaces::World, spaces::World> = Mat::identity().translate(5, -3, 2);
        let p = Point::new(-3, 4, 5);
        assert_relative_eq!(transform.inverse() * p, Point::new(-8, 7, 3));
    }

    #[test]
    fn trans_does_not_affect_vectors() {
        let transform: Mat<4, spaces::World, spaces::World> = Mat::identity().translate(5, -3, 2);
        let v = Vector::new(-3, 4, 5);
        assert_relative_eq!(transform * v, v);
    }

    #[test]
    fn scaling_pt() {
        let transform: Mat<4, spaces::World, spaces::World> = Mat::identity().scale(2, 3, 4);
        let p = Point::new(-4, 6, 8);
        assert_relative_eq!(transform * p, Point::new(-8, 18, 32));
    }

    #[test]
    fn scaling_vec() {
        let transform: Mat<4, spaces::World, spaces::World> = Mat::identity().scale(2, 3, 4);
        let p = Vector::new(-4, 6, 8);
        assert_relative_eq!(transform * p, Vector::new(-8, 18, 32));
    }

    #[test]
    fn scaling_vec_inv() {
        let transform: Mat<4, spaces::World, spaces::World> = Mat::identity().scale(2, 3, 4);
        let p = Vector::new(-4, 6, 8);
        assert_relative_eq!(transform.inverse() * p, Vector::new(-2, 2, 2));
    }

    #[test]
    fn scaling_reflect() {
        let transform: Mat<4, spaces::World, spaces::World> = Mat::identity().scale(-1, 1, 1);
        let p = Point::new(2, 3, 4);
        assert_relative_eq!(transform * p, Point::new(-2, 3, 4));
    }

    #[test]
    fn rot_x() {
        let p: Point<spaces::World> = Point::new(0, 1, 0);
        let id: Mat<4, spaces::World, spaces::World> = Mat::identity();
        assert_relative_eq!(
            id.rotate_x(PI / 4.0) * p,
            Point::new(0, 2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0)
        );
        assert_relative_eq!(id.rotate_x(PI / 2.0) * p, Point::new(0, 0, 1));
    }

    #[test]
    fn rot_y() {
        let p: Point<spaces::World> = Point::new(0, 0, 1);
        let id: Mat<4, spaces::World, spaces::World> = Mat::identity();
        assert_relative_eq!(
            id.rotate_y(PI / 4.0) * p,
            Point::new(2f64.sqrt() / 2.0, 0, 2f64.sqrt() / 2.0)
        );
        assert_relative_eq!(id.rotate_y(PI / 2.0) * p, Point::new(1, 0, 0));
    }

    #[test]
    fn rot_z() {
        let p: Point<spaces::World> = Point::new(0, 1, 0);
        let id: Mat<4, spaces::World, spaces::World> = Mat::identity();
        assert_relative_eq!(
            id.rotate_z(PI / 4.0) * p,
            Point::new(-2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0, 0)
        );
        assert_relative_eq!(id.rotate_z(PI / 2.0) * p, Point::new(-1, 0, 0));
    }

    #[test]
    fn shear() {
        let p: Point<spaces::World> = Point::new(2, 3, 4);
        let id: Mat<4, spaces::World, spaces::World> = Mat::identity();
        assert_relative_eq!(id.shear(1, 0, 0, 0, 0, 0) * p, Point::new(5, 3, 4));
        assert_relative_eq!(id.shear(0, 1, 0, 0, 0, 0) * p, Point::new(6, 3, 4));
        assert_relative_eq!(id.shear(0, 0, 1, 0, 0, 0) * p, Point::new(2, 5, 4));
        assert_relative_eq!(id.shear(0, 0, 0, 1, 0, 0) * p, Point::new(2, 7, 4));
        assert_relative_eq!(id.shear(0, 0, 0, 0, 1, 0) * p, Point::new(2, 3, 6));
        assert_relative_eq!(id.shear(0, 0, 0, 0, 0, 1) * p, Point::new(2, 3, 7));
    }

    #[test]
    fn combined_xforms() {
        let p: Point<spaces::World> = Point::new(1, 0, 1);
        let m: Mat<4, spaces::World, spaces::World> = Mat::identity().rotate_x(PI / 2.0);
        assert_relative_eq!(m * p, Point::new(1, -1, 0));

        let m: Mat<4, spaces::World, spaces::World> = m.scale(5, 5, 5);
        assert!(Relative {
            epsilon: 0.00001,
            max_relative: 0.00001
        }
        .eq(&(m * p), &Point::new(5, -5, 0)));

        let m: Mat<4, spaces::World, spaces::World> = m.translate(10, 5, 7);
        assert_relative_eq!(m * p, Point::new(15, 0, 7));
        assert!(Relative {
            epsilon: 0.00001,
            max_relative: 0.00001
        }
        .eq(&(m * p), &Point::new(15, 0, 7)));
    }
}
