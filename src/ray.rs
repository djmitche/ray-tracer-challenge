use crate::{Mat, Space, Tup};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray<S: Space> {
    pub origin: Tup<S>,
    pub direction: Tup<S>,
}

impl<S: Space> Ray<S> {
    pub fn new(origin: Tup<S>, direction: Tup<S>) -> Self {
        debug_assert!(origin.is_point());
        debug_assert!(direction.is_vector());
        Self { origin, direction }
    }

    pub fn position<T: Into<f64>>(&self, t: T) -> Tup<S> {
        self.origin + self.direction * t.into()
    }
}

impl<S1: Space, S2: Space> std::ops::Mul<Ray<S1>> for Mat<4, S1, S2> {
    type Output = Ray<S2>;
    fn mul(self, other: Ray<S1>) -> Ray<S2> {
        Ray::new(self * other.origin, self * other.direction)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::spaces;
    use approx::*;

    #[test]
    fn create_query_ray() {
        let r: Ray<spaces::World> = Ray::new(Tup::point(1, 2, 3), Tup::vector(4, 5, 6));
        assert_relative_eq!(r.origin, Tup::point(1, 2, 3));
        assert_relative_eq!(r.direction, Tup::vector(4, 5, 6));
    }

    #[test]
    fn point_from_distance() {
        let r: Ray<spaces::World> = Ray::new(Tup::point(2, 3, 4), Tup::vector(1, 0, 0));
        assert_relative_eq!(r.position(0), Tup::point(2, 3, 4));
        assert_relative_eq!(r.position(1), Tup::point(3, 3, 4));
        assert_relative_eq!(r.position(-1), Tup::point(1, 3, 4));
        assert_relative_eq!(r.position(2.5), Tup::point(4.5, 3, 4));
    }

    #[test]
    fn translate_ray() {
        let r: Ray<spaces::World> = Ray::new(Tup::point(1, 2, 3), Tup::vector(0, 1, 0));
        let m: Mat<4, spaces::World, spaces::World> = Mat::identity().translate(3, 4, 5);
        let r2 = m * r;
        assert_relative_eq!(r2.origin, Tup::point(4, 6, 8));
        assert_relative_eq!(r2.direction, Tup::vector(0, 1, 0));
    }

    #[test]
    fn scale_ray() {
        let r: Ray<spaces::World> = Ray::new(Tup::point(1, 2, 3), Tup::vector(0, 1, 0));
        let m: Mat<4, spaces::World, spaces::Object> = Mat::identity().scale(2, 3, 4);
        let r2: Ray<spaces::Object> = m * r;
        assert_relative_eq!(r2.origin, Tup::point(2, 6, 12));
        assert_relative_eq!(r2.direction, Tup::vector(0, 3, 0));
    }
}
