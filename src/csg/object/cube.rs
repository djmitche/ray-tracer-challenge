use crate::csg::ObjectInner;
use crate::{spaces, Intersections, ObjectIndex, Point, Ray, Vector};

/// A plane in x-z
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cube;

impl ObjectInner for Cube {
    fn intersect(
        &self,
        object_index: ObjectIndex,
        ray: Ray<spaces::Object>,
        inters: &mut Intersections,
    ) {
        fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
            let tmin_num = -1.0 - origin;
            let tmax_num = 1.0 - origin;

            let (tmin, tmax) = if direction.abs() >= f64::EPSILON {
                (tmin_num / direction, tmax_num / direction)
            } else {
                (tmin_num * f64::INFINITY, tmax_num * f64::INFINITY)
            };
            if tmin < tmax {
                (tmin, tmax)
            } else {
                (tmax, tmin)
            }
        }

        let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z);

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);
        if tmin <= tmax {
            inters.add(tmin, object_index);
            inters.add(tmax, object_index);
        }
    }

    fn normal(&self, point: Point<spaces::Object>) -> Vector<spaces::Object> {
        let x = point.x.abs();
        let y = point.y.abs();
        let z = point.z.abs();

        if x >= y {
            if x >= z {
                Vector::new(point.x, 0, 0)
            } else {
                Vector::new(0, 0, point.z)
            }
        } else {
            if y >= z {
                Vector::new(0, point.y, 0)
            } else {
                Vector::new(0, 0, point.z)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::csg::*;
    use crate::*;
    use approx::*;

    macro_rules! test_intersect {
        ($name:ident, $origin:expr, $direction:expr, $t1:expr, $t2:expr) => {
            #[test]
            fn $name() {
                let ray = Ray::new($origin, $direction);
                let oi = ObjectIndex::test_value(7);
                let mut inters = Intersections::default();
                Cube.intersect(oi, ray, &mut inters);
                let mut it = inters.iter();
                assert_relative_eq!(
                    it.next().expect("expected first intersection").t,
                    $t1.into()
                );
                assert_relative_eq!(
                    it.next().expect("expected second intersection").t,
                    $t2.into()
                );
                assert!(it.next().is_none());
            }
        };
        ($name:ident, $origin:expr, $direction:expr) => {
            #[test]
            fn $name() {
                let ray = Ray::new($origin, $direction);
                let oi = ObjectIndex::test_value(7);
                let mut inters = Intersections::default();
                Cube.intersect(oi, ray, &mut inters);
                let mut it = inters.iter();
                assert!(it.next().is_none());
            }
        };
    }

    test_intersect!(pos_x, Point::new(5, 0.5, 0), Vector::new(-1, 0, 0), 4, 6);
    test_intersect!(neg_x, Point::new(-5, 0.5, 0), Vector::new(1, 0, 0), 4, 6);
    test_intersect!(pos_y, Point::new(0.5, 5, 0), Vector::new(0, -1, 0), 4, 6);
    test_intersect!(neg_y, Point::new(0.5, -5, 0), Vector::new(0, 1, 0), 4, 6);
    test_intersect!(pos_z, Point::new(0.5, 0, 5), Vector::new(0, 0, -1), 4, 6);
    test_intersect!(neg_z, Point::new(0.5, 0, -5), Vector::new(0, 0, 1), 4, 6);
    test_intersect!(inside, Point::new(0, 0.5, 0), Vector::new(0, 0, 1), -1, 1);
    test_intersect!(
        miss_oblique_x,
        Point::new(-2, 0, 0),
        Vector::new(0.2673, 0.5345, 0.8018)
    );
    test_intersect!(
        miss_oblique_y,
        Point::new(0, -2, 0),
        Vector::new(0.8018, 0.2673, 0.5345)
    );
    test_intersect!(
        miss_oblique_z,
        Point::new(0, 0, -2),
        Vector::new(0.5345, 0.8018, 0.2673)
    );
    test_intersect!(miss_z, Point::new(2, 0, 2), Vector::new(0, 0, -1));
    test_intersect!(miss_y, Point::new(0, 2, 2), Vector::new(0, -1, 0));
    test_intersect!(miss_x, Point::new(2, 2, 0), Vector::new(-1, 0, 0));

    macro_rules! test_norm {
        ($name:ident, $point:expr, $normal:expr) => {
            #[test]
            fn $name() {
                let normal = Cube.normal($point);
                assert_relative_eq!(normal, $normal);
            }
        };
    }

    test_norm!(norm_pos_x, Point::new(1, 0.5, -0.8), Vector::new(1, 0, 0));
    test_norm!(norm_neg_x, Point::new(-1, -0.2, 0.9), Vector::new(-1, 0, 0));
    test_norm!(norm_pos_y, Point::new(-0.4, 1, -0.1), Vector::new(0, 1, 0));
    test_norm!(norm_neg_y, Point::new(0.3, -1, -0.7), Vector::new(0, -1, 0));
    test_norm!(norm_pos_z, Point::new(-0.6, 0.3, 1), Vector::new(0, 0, 1));
    test_norm!(norm_neg_z, Point::new(0.4, 0.4, -1), Vector::new(0, 0, -1));
    test_norm!(norm_pos_corner, Point::new(1, 1, 1), Vector::new(1, 0, 0));
    test_norm!(
        norm_neg_corner,
        Point::new(-1, -1, -1),
        Vector::new(-1, 0, 0)
    );
}
