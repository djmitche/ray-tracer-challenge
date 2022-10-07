use crate::Ray;
use crate::Tup;
use crate::{Intersection, Object};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    pub center: Tup,
    pub radius: f64,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Tup::point(0, 0, 0),
            radius: 1.0,
        }
    }
}

impl Object for Sphere {
    fn intersect<'o>(&'o self, ray: &Ray) -> Vec<Intersection<'o>> {
        // TODO: accept an array & return a slice of it
        let sphere_to_ray = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        }

        vec![
            Intersection {
                t: (-b - discriminant.sqrt()) / (a * 2.0),
                obj: self,
            },
            Intersection {
                t: (-b + discriminant.sqrt()) / (a * 2.0),
                obj: self,
            },
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;

    #[test]
    fn ray_intersects_sphere() {
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let s = Sphere::default();

        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        println!("{:?}", xs);
        assert_relative_eq!(xs[0].t, 4.0);
        assert_relative_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_one_point() {
        let r = Ray::new(Tup::point(0, 1, -5), Tup::vector(0, 0, 1));
        let s = Sphere::default();

        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].t, 5.0);
        assert_relative_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_intersects_sphere_zero_points() {
        let r = Ray::new(Tup::point(0, 2, -5), Tup::vector(0, 0, 1));
        let s = Sphere::default();

        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_origin_in_sphere() {
        let r = Ray::new(Tup::point(0, 0, 0), Tup::vector(0, 0, 1));
        let s = Sphere::default();

        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].t, -1.0);
        assert_relative_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Tup::point(0, 0, 5), Tup::vector(0, 0, 1));
        let s = Sphere::default();

        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].t, -6.0);
        assert_relative_eq!(xs[1].t, -4.0);
    }
}
