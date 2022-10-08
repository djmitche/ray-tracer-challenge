use crate::{Intersection, Intersections, Object};
use crate::{Mat, Ray, Tup};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    pub center: Tup,
    pub radius: f64,
    pub transform: Mat<4>,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Tup::point(0, 0, 0),
            radius: 1.0,
            transform: Mat::identity(),
        }
    }
}

impl Sphere {
    pub fn with_transform(transform: Mat<4>) -> Self {
        Self {
            transform,
            ..Default::default()
        }
    }
}

impl Object for Sphere {
    fn intersect<'o>(&'o self, ray: &Ray) -> Intersections<'o> {
        let ray = self.transform.inverse() * *ray;
        // TODO: accept an array & return a slice of it
        let sphere_to_ray = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        let mut inters = Intersections::default();
        if discriminant < 0.0 {
            return inters;
        }

        inters.add(Intersection::new(
            (-b - discriminant.sqrt()) / (a * 2.0),
            self,
        ));
        inters.add(Intersection::new(
            (-b + discriminant.sqrt()) / (a * 2.0),
            self,
        ));
        inters
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;

    #[test]
    fn construct_sphere_default() {
        let s = Sphere::default();
        assert_relative_eq!(s.transform, Mat::identity());
    }

    #[test]
    fn construct_sphere_with_transform() {
        let xf = Mat::identity().translate(1, 2, 3);
        let s = Sphere::with_transform(xf);
        assert_relative_eq!(s.transform, xf);
    }

    #[test]
    fn ray_intersects_sphere() {
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let s = Sphere::default();

        let xs = s.intersect(&r);
        let mut it = xs.iter();
        assert_relative_eq!(it.next().expect("intersection").t, 4.0);
        assert_relative_eq!(it.next().expect("intersection").t, 6.0);
        assert!(it.next().is_none());
    }

    #[test]
    fn ray_intersects_sphere_one_point() {
        let r = Ray::new(Tup::point(0, 1, -5), Tup::vector(0, 0, 1));
        let s = Sphere::default();

        let xs = s.intersect(&r);
        let mut it = xs.iter();
        assert_relative_eq!(it.next().expect("intersection").t, 5.0);
        assert_relative_eq!(it.next().expect("intersection").t, 5.0);
        assert!(it.next().is_none());
    }

    #[test]
    fn ray_intersects_sphere_zero_points() {
        let r = Ray::new(Tup::point(0, 2, -5), Tup::vector(0, 0, 1));
        let s = Sphere::default();

        let xs = s.intersect(&r);
        let mut it = xs.iter();
        assert!(it.next().is_none());
    }

    #[test]
    fn ray_origin_in_sphere() {
        let r = Ray::new(Tup::point(0, 0, 0), Tup::vector(0, 0, 1));
        let s = Sphere::default();

        let xs = s.intersect(&r);
        let mut it = xs.iter();
        assert_relative_eq!(it.next().expect("intersection").t, -1.0);
        assert_relative_eq!(it.next().expect("intersection").t, 1.0);
        assert!(it.next().is_none());
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Tup::point(0, 0, 5), Tup::vector(0, 0, 1));
        let s = Sphere::default();

        let xs = s.intersect(&r);
        let mut it = xs.iter();
        assert_relative_eq!(it.next().expect("intersection").t, -6.0);
        assert_relative_eq!(it.next().expect("intersection").t, -4.0);
        assert!(it.next().is_none());
    }

    #[test]
    fn ray_intersects_scaled_sphere() {
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let s = Sphere::with_transform(Mat::identity().scale(2, 2, 2));

        let xs = s.intersect(&r);
        let mut it = xs.iter();
        assert_relative_eq!(it.next().expect("intersection").t, 3.0);
        assert_relative_eq!(it.next().expect("intersection").t, 7.0);
        assert!(it.next().is_none());
    }

    #[test]
    fn ray_intersects_translated_sphere() {
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let s = Sphere::with_transform(Mat::identity().translate(5, 0, 0));

        let xs = s.intersect(&r);
        let mut it = xs.iter();
        assert!(it.next().is_none());
    }
}
