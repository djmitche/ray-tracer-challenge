use crate::{Intersection, Object, Ray, Tup};

pub struct Comps<'o> {
    /// True if the eye is looking from inside the shape
    pub inside: bool,

    /// The intersected object
    pub obj: &'o dyn Object,

    /// The point along the ray at which the object occurs
    pub t: f64,

    /// The point at which the intersection occurs
    pub point: Tup,

    /// The vector from this point to the eye
    pub eyev: Tup,

    /// The normal vector at the point
    pub normalv: Tup,
}

impl<'o> Comps<'o> {
    pub fn prepare(intersection: &Intersection<'o>, ray: &Ray) -> Self {
        let point = ray.position(intersection.t);
        let eyev = -ray.direction;
        let mut normalv = intersection.obj.normal(point);
        let inside = if normalv.dot(eyev) < 0.0 {
            // use the inside surface, with the opposite normal
            normalv = -normalv;
            true
        } else {
            false
        };
        Self {
            inside,
            obj: intersection.obj,
            t: intersection.t,
            point,
            eyev,
            normalv,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Sphere;
    use approx::*;

    #[test]
    fn precompute_state() {
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let shape = Sphere::default();
        let i = Intersection {
            obj: &shape,
            t: 4.0,
        };

        let comps = Comps::prepare(&i, &r);
        assert_eq!(comps.inside, false);
        assert_relative_eq!(comps.t, 4.0);
        assert_relative_eq!(comps.point, Tup::point(0, 0, -1));
        assert_relative_eq!(comps.eyev, Tup::vector(0, 0, -1));
        assert_relative_eq!(comps.normalv, Tup::vector(0, 0, -1));
    }

    #[test]
    fn precompute_state_inside() {
        let r = Ray::new(Tup::point(0, 0, 0), Tup::vector(0, 0, 1));
        let shape = Sphere::default();
        let i = Intersection {
            obj: &shape,
            t: 1.0,
        };

        let comps = Comps::prepare(&i, &r);
        assert_eq!(comps.inside, true);
        assert_relative_eq!(comps.t, 1.0);
        assert_relative_eq!(comps.point, Tup::point(0, 0, 1));
        assert_relative_eq!(comps.eyev, Tup::vector(0, 0, -1));
        assert_relative_eq!(comps.normalv, Tup::vector(0, 0, -1));
    }
}
