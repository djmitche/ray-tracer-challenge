use crate::{Mat, Point, Space, Vector};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray<S: Space> {
    pub origin: Point<S>,
    pub direction: Vector<S>,
}

impl<S: Space> Ray<S> {
    pub fn new(origin: Point<S>, direction: Vector<S>) -> Self {
        Self { origin, direction }
    }

    pub fn position<T: Into<f64>>(&self, t: T) -> Point<S> {
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
        let r: Ray<spaces::World> = Ray::new(Point::new(1, 2, 3), Vector::new(4, 5, 6));
        assert_relative_eq!(r.origin, Point::new(1, 2, 3));
        assert_relative_eq!(r.direction, Vector::new(4, 5, 6));
    }

    #[test]
    fn point_from_distance() {
        let r: Ray<spaces::World> = Ray::new(Point::new(2, 3, 4), Vector::new(1, 0, 0));
        assert_relative_eq!(r.position(0), Point::new(2, 3, 4));
        assert_relative_eq!(r.position(1), Point::new(3, 3, 4));
        assert_relative_eq!(r.position(-1), Point::new(1, 3, 4));
        assert_relative_eq!(r.position(2.5), Point::new(4.5, 3, 4));
    }

    #[test]
    fn translate_ray() {
        let r: Ray<spaces::World> = Ray::new(Point::new(1, 2, 3), Vector::new(0, 1, 0));
        let m: Mat<4, spaces::World, spaces::World> = Mat::identity().translate(3, 4, 5);
        let r2 = m * r;
        assert_relative_eq!(r2.origin, Point::new(4, 6, 8));
        assert_relative_eq!(r2.direction, Vector::new(0, 1, 0));
    }

    #[test]
    fn scale_ray() {
        let r: Ray<spaces::World> = Ray::new(Point::new(1, 2, 3), Vector::new(0, 1, 0));
        let m: Mat<4, spaces::World, spaces::Object> = Mat::identity().scale(2, 3, 4);
        let r2: Ray<spaces::Object> = m * r;
        assert_relative_eq!(r2.origin, Point::new(2, 6, 12));
        assert_relative_eq!(r2.direction, Vector::new(0, 3, 0));
    }
}
