use crate::{Intersection, Intersections, Mat, Material, Object, Ray, Tup};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    inv_transform: Mat<4>,
    inv_transp_transform: Mat<4>,
    material: Material,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            inv_transform: Mat::identity(),
            inv_transp_transform: Mat::identity(),
            material: Material::default(),
        }
    }
}

impl Sphere {
    pub fn with_transform(mut self, transform: Mat<4>) -> Self {
        self.inv_transform = transform.inverse();
        self.inv_transp_transform = self.inv_transform.transpose();
        self
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }
}

impl Object for Sphere {
    fn intersect<'o>(&'o self, ray: &Ray, inters: &mut Intersections<'o>) {
        let ray = self.inv_transform * *ray;
        let sphere_to_ray = ray.origin.as_vector();
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            inters.add(Intersection::new(
                (-b - discriminant.sqrt()) / (a * 2.0),
                self,
            ));
            inters.add(Intersection::new(
                (-b + discriminant.sqrt()) / (a * 2.0),
                self,
            ));
        }
    }

    fn normal(&self, point: Tup) -> Tup {
        let object_point = self.inv_transform * point;
        let object_normal = object_point.as_vector();
        let world_normal = self.inv_transp_transform * object_normal;
        world_normal.as_vector().normalize()
    }

    fn material(&self) -> &Material {
        &self.material
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Tup;
    use approx::*;
    use std::f64::consts::PI;

    #[test]
    fn construct_sphere_default() {
        let s = Sphere::default();
        assert_relative_eq!(s.inv_transform, Mat::identity());
        assert_relative_eq!(s.material().ambient, 0.1);
    }

    #[test]
    fn construct_sphere_with_transform() {
        let xf = Mat::identity().translate(1, 2, 3);
        let s = Sphere::default().with_transform(xf);
        assert_relative_eq!(s.inv_transform, xf.inverse());
    }

    #[test]
    fn construct_sphere_with_material() {
        let s = Sphere::default().with_material(Material {
            ambient: 29.0,
            ..Default::default()
        });
        assert_relative_eq!(s.material.ambient, 29.0);
    }

    #[test]
    fn ray_intersects_sphere() {
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let s = Sphere::default();

        let mut xs = Intersections::default();
        s.intersect(&r, &mut xs);
        let mut it = xs.iter();
        assert_relative_eq!(it.next().expect("intersection").t, 4.0);
        assert_relative_eq!(it.next().expect("intersection").t, 6.0);
        assert!(it.next().is_none());
    }

    #[test]
    fn ray_intersects_sphere_one_point() {
        let r = Ray::new(Tup::point(0, 1, -5), Tup::vector(0, 0, 1));
        let s = Sphere::default();

        let mut xs = Intersections::default();
        s.intersect(&r, &mut xs);
        let mut it = xs.iter();
        assert_relative_eq!(it.next().expect("intersection").t, 5.0);
        assert_relative_eq!(it.next().expect("intersection").t, 5.0);
        assert!(it.next().is_none());
    }

    #[test]
    fn ray_intersects_sphere_zero_points() {
        let r = Ray::new(Tup::point(0, 2, -5), Tup::vector(0, 0, 1));
        let s = Sphere::default();

        let mut xs = Intersections::default();
        s.intersect(&r, &mut xs);
        let mut it = xs.iter();
        assert!(it.next().is_none());
    }

    #[test]
    fn ray_origin_in_sphere() {
        let r = Ray::new(Tup::point(0, 0, 0), Tup::vector(0, 0, 1));
        let s = Sphere::default();

        let mut xs = Intersections::default();
        s.intersect(&r, &mut xs);
        let mut it = xs.iter();
        assert_relative_eq!(it.next().expect("intersection").t, -1.0);
        assert_relative_eq!(it.next().expect("intersection").t, 1.0);
        assert!(it.next().is_none());
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Tup::point(0, 0, 5), Tup::vector(0, 0, 1));
        let s = Sphere::default();

        let mut xs = Intersections::default();
        s.intersect(&r, &mut xs);
        let mut it = xs.iter();
        assert_relative_eq!(it.next().expect("intersection").t, -6.0);
        assert_relative_eq!(it.next().expect("intersection").t, -4.0);
        assert!(it.next().is_none());
    }

    #[test]
    fn ray_intersects_scaled_sphere() {
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let s = Sphere::default().with_transform(Mat::identity().scale(2, 2, 2));

        let mut xs = Intersections::default();
        s.intersect(&r, &mut xs);
        let mut it = xs.iter();
        assert_relative_eq!(it.next().expect("intersection").t, 3.0);
        assert_relative_eq!(it.next().expect("intersection").t, 7.0);
        assert!(it.next().is_none());
    }

    #[test]
    fn ray_intersects_translated_sphere() {
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let s = Sphere::default().with_transform(Mat::identity().translate(5, 0, 0));

        let mut xs = Intersections::default();
        s.intersect(&r, &mut xs);
        let mut it = xs.iter();
        assert!(it.next().is_none());
    }

    #[test]
    fn sphere_normal() {
        let s = Sphere::default();

        assert_relative_eq!(s.normal(Tup::point(1, 0, 0)), Tup::vector(1, 0, 0));
        assert_relative_eq!(s.normal(Tup::point(0, 1, 0)), Tup::vector(0, 1, 0));
        assert_relative_eq!(s.normal(Tup::point(0, 0, 1)), Tup::vector(0, 0, 1));
        let rt3over3 = 3f64.sqrt() / 3.0;
        assert_relative_eq!(
            s.normal(Tup::point(rt3over3, rt3over3, rt3over3)),
            Tup::vector(rt3over3, rt3over3, rt3over3)
        );
    }

    #[test]
    fn translated_sphere_normal() {
        let s = Sphere::default().with_transform(Mat::identity().translate(0, 1, 0));

        assert_relative_eq!(
            s.normal(Tup::point(0, 1.7071067811865475, -0.7071067811865475)),
            Tup::vector(0, 0.7071067811865475, -0.7071067811865475)
        );
    }

    #[test]
    fn transformed_sphere_normal() {
        let s =
            Sphere::default().with_transform(Mat::identity().scale(1, 0.5, 1).rotate_y(PI / 5.0));

        let rt2over2 = 2f64.sqrt() / 2.0;
        assert_relative_eq!(
            s.normal(Tup::point(0, rt2over2, -rt2over2)),
            Tup::vector(0, 0.9701425001453319, -0.24253562503633302)
        );
    }
}
