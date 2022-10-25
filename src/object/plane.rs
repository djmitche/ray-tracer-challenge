use crate::{spaces, Intersections, ObjectInner, Point, Ray, Vector};

/// A plane in x-z
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Plane;

impl ObjectInner for Plane {
    fn intersect<'o>(&'o self, ray: Ray<spaces::Object>, inters: &mut Intersections<'o>) {
        if ray.direction.y.abs() < 0.00001 {
            return;
        }

        inters.add(-ray.origin.y / ray.direction.y);
    }

    fn normal(&self, _point: Point<spaces::Object>) -> Vector<spaces::Object> {
        Vector::new(0, 1, 0)
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use approx::*;

    #[test]
    fn intersect_parallel() {
        let r = Ray::new(Point::new(0, 10, 0), Vector::new(0, 0, 1));

        let mut xs = Intersections::default();
        let o = Object::new(Plane);
        xs.set_object(&o);
        Plane.intersect(r, &mut xs);
        let mut it = xs.iter();
        assert!(it.next().is_none());
    }

    #[test]
    fn intersect_coplanar() {
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));

        let mut xs = Intersections::default();
        let o = Object::new(Plane);
        xs.set_object(&o);
        Plane.intersect(r, &mut xs);
        let mut it = xs.iter();
        assert!(it.next().is_none());
    }

    #[test]
    fn intersect_from_above() {
        let r = Ray::new(Point::new(0, 1, 0), Vector::new(0, -1, 0));

        let mut xs = Intersections::default();
        let o = Object::new(Plane);
        xs.set_object(&o);
        Plane.intersect(r, &mut xs);
        let mut it = xs.iter();
        assert_relative_eq!(it.next().expect("intersection").t, 1.0);
        assert!(it.next().is_none());
    }

    #[test]
    fn intersect_from_below() {
        let r = Ray::new(Point::new(0, -1, 0), Vector::new(0, 1, 0));

        let mut xs = Intersections::default();
        let o = Object::new(Plane);
        xs.set_object(&o);
        Plane.intersect(r, &mut xs);
        let mut it = xs.iter();
        assert_relative_eq!(it.next().expect("intersection").t, 1.0);
        assert!(it.next().is_none());
    }

    #[test]
    fn norma() {
        assert_relative_eq!(Plane.normal(Point::new(13, 0, 11)), Vector::new(0, 1, 0));
    }
}
