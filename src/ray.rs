use crate::Sphere;
use crate::Tup;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Tup,
    pub direction: Tup,
}

impl Ray {
    pub fn new(origin: Tup, direction: Tup) -> Self {
        debug_assert!(origin.is_point());
        debug_assert!(direction.is_vector());
        Self { origin, direction }
    }

    pub fn position<T: Into<f64>>(&self, t: T) -> Tup {
        self.origin + self.direction * t.into()
    }

    // TODO: Intersectible trait
    // TODO: accept an array & return a slice of it
    pub fn intersect_sphere(&self, s: &Sphere) -> Vec<f64> {
        let sphere_to_ray = self.origin - s.center;
        let a = self.direction.dot(self.direction);
        let b = 2.0 * self.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        }

        vec![
            (-b - discriminant.sqrt()) / (a * 2.0),
            (-b + discriminant.sqrt()) / (a * 2.0),
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;

    #[test]
    fn create_query_ray() {
        let r = Ray::new(Tup::point(1, 2, 3), Tup::vector(4, 5, 6));
        assert_relative_eq!(r.origin, Tup::point(1, 2, 3));
        assert_relative_eq!(r.direction, Tup::vector(4, 5, 6));
    }

    #[test]
    fn point_from_distance() {
        let r = Ray::new(Tup::point(2, 3, 4), Tup::vector(1, 0, 0));
        assert_relative_eq!(r.position(0), Tup::point(2, 3, 4));
        assert_relative_eq!(r.position(1), Tup::point(3, 3, 4));
        assert_relative_eq!(r.position(-1), Tup::point(1, 3, 4));
        assert_relative_eq!(r.position(2.5), Tup::point(4.5, 3, 4));
    }

    #[test]
    fn ray_intersects_sphere() {
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let s = Sphere::default();

        let xs = r.intersect_sphere(&s);
        assert_eq!(xs.len(), 2);
        println!("{:?}", xs);
        assert_relative_eq!(xs[0], 4.0);
        assert_relative_eq!(xs[1], 6.0);
    }

    #[test]
    fn ray_intersects_sphere_one_point() {
        let r = Ray::new(Tup::point(0, 1, -5), Tup::vector(0, 0, 1));
        let s = Sphere::default();

        let xs = r.intersect_sphere(&s);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0], 5.0);
        assert_relative_eq!(xs[1], 5.0);
    }

    #[test]
    fn ray_intersects_sphere_zero_points() {
        let r = Ray::new(Tup::point(0, 2, -5), Tup::vector(0, 0, 1));
        let s = Sphere::default();

        let xs = r.intersect_sphere(&s);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_origin_in_sphere() {
        let r = Ray::new(Tup::point(0, 0, 0), Tup::vector(0, 0, 1));
        let s = Sphere::default();

        let xs = r.intersect_sphere(&s);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0], -1.0);
        assert_relative_eq!(xs[1], 1.0);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Tup::point(0, 0, 5), Tup::vector(0, 0, 1));
        let s = Sphere::default();

        let xs = r.intersect_sphere(&s);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0], -6.0);
        assert_relative_eq!(xs[1], -4.0);
    }
}
