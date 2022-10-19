use crate::{spaces, Intersections, ObjectInner, Point, Ray, Vector};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere;

impl ObjectInner for Sphere {
    fn intersect<'o>(&'o self, ray: Ray<spaces::Object>, inters: &mut Intersections<'o>) {
        let sphere_to_ray = ray.origin.as_vector();
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            let sqrt = discriminant.sqrt();
            inters.add((-b - sqrt) / (a * 2.0));
            inters.add((-b + sqrt) / (a * 2.0));
        }
    }

    fn normal(&self, point: Point<spaces::Object>) -> Vector<spaces::Object> {
        point.as_vector()
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use approx::*;
    use std::f64::consts::PI;

    #[test]
    fn ray_intersects_sphere() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let s = Object::new(Sphere);

        let mut xs = Intersections::default();
        s.intersect(&r, &mut xs);
        let mut it = xs.iter();
        assert_relative_eq!(it.next().expect("intersection").t, 4.0);
        assert_relative_eq!(it.next().expect("intersection").t, 6.0);
        assert!(it.next().is_none());
    }

    #[test]
    fn ray_intersects_sphere_one_point() {
        let r = Ray::new(Point::new(0, 1, -5), Vector::new(0, 0, 1));
        let s = Object::new(Sphere);

        let mut xs = Intersections::default();
        s.intersect(&r, &mut xs);
        let mut it = xs.iter();
        assert_relative_eq!(it.next().expect("intersection").t, 5.0);
        assert_relative_eq!(it.next().expect("intersection").t, 5.0);
        assert!(it.next().is_none());
    }

    #[test]
    fn ray_intersects_sphere_zero_points() {
        let r = Ray::new(Point::new(0, 2, -5), Vector::new(0, 0, 1));
        let s = Object::new(Sphere);

        let mut xs = Intersections::default();
        s.intersect(&r, &mut xs);
        let mut it = xs.iter();
        assert!(it.next().is_none());
    }

    #[test]
    fn ray_origin_in_sphere() {
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let s = Object::new(Sphere);

        let mut xs = Intersections::default();
        s.intersect(&r, &mut xs);
        let mut it = xs.iter();
        assert_relative_eq!(it.next().expect("intersection").t, -1.0);
        assert_relative_eq!(it.next().expect("intersection").t, 1.0);
        assert!(it.next().is_none());
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Point::new(0, 0, 5), Vector::new(0, 0, 1));
        let s = Object::new(Sphere);

        let mut xs = Intersections::default();
        s.intersect(&r, &mut xs);
        let mut it = xs.iter();
        assert_relative_eq!(it.next().expect("intersection").t, -6.0);
        assert_relative_eq!(it.next().expect("intersection").t, -4.0);
        assert!(it.next().is_none());
    }

    #[test]
    fn ray_intersects_scaled_sphere() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let s = Object::new(Sphere).with_transform(Mat::identity().scale(2, 2, 2));

        let mut xs = Intersections::default();
        s.intersect(&r, &mut xs);
        let mut it = xs.iter();
        assert_relative_eq!(it.next().expect("intersection").t, 3.0);
        assert_relative_eq!(it.next().expect("intersection").t, 7.0);
        assert!(it.next().is_none());
    }

    #[test]
    fn ray_intersects_translated_sphere() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let s = Object::new(Sphere).with_transform(Mat::identity().translate(5, 0, 0));

        let mut xs = Intersections::default();
        s.intersect(&r, &mut xs);
        let mut it = xs.iter();
        assert!(it.next().is_none());
    }

    #[test]
    fn sphere_normal() {
        let s = Object::new(Sphere);

        assert_relative_eq!(s.normal(Point::new(1, 0, 0)), Vector::new(1, 0, 0));
        assert_relative_eq!(s.normal(Point::new(0, 1, 0)), Vector::new(0, 1, 0));
        assert_relative_eq!(s.normal(Point::new(0, 0, 1)), Vector::new(0, 0, 1));
        let rt3over3 = 3f64.sqrt() / 3.0;
        assert_relative_eq!(
            s.normal(Point::new(rt3over3, rt3over3, rt3over3)),
            Vector::new(rt3over3, rt3over3, rt3over3)
        );
    }

    #[test]
    fn translated_sphere_normal() {
        let s = Object::new(Sphere).with_transform(Mat::identity().translate(0, 1, 0));

        assert_relative_eq!(
            s.normal(Point::new(0, 1.7071067811865475, -0.7071067811865475)),
            Vector::new(0, 0.7071067811865475, -0.7071067811865475)
        );
    }

    #[test]
    fn transformed_sphere_normal() {
        let s =
            Object::new(Sphere).with_transform(Mat::identity().scale(1, 0.5, 1).rotate_y(PI / 5.0));

        let rt2over2 = 2f64.sqrt() / 2.0;
        assert_relative_eq!(
            s.normal(Point::new(0, rt2over2, -rt2over2)),
            Vector::new(0, 0.9701425001453319, -0.24253562503633302)
        );
    }
}
