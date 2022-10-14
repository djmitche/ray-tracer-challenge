use crate::{Color, Intersection, Object, Ray, Tup, World};

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

pub fn shade_hit(world: &World, comps: &Comps) -> Color {
    world
        .light
        .lighting(comps.obj.material(), comps.point, comps.eyev, comps.normalv)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Intersections, Light, Mat, Material, Sphere};
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

    #[test]
    fn shade_intersection() {
        let w = World::test_world();
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));

        let mut inters = Intersections::default();
        w.intersect(&r, &mut inters);
        let i = *inters.hit().unwrap();

        let comps = Comps::prepare(&i, &r);
        let c = shade_hit(&w, &comps);
        assert_relative_eq!(
            c,
            Color::new(
                0.38066119308103435,
                0.47582649135129296,
                0.28549589481077575
            )
        );
    }

    #[test]
    fn shade_intersection_inside() {
        let mut w = World::test_world();
        w.light = Light::new_point(Tup::point(0, 0.25, 0), Color::white());
        let r = Ray::new(Tup::point(0, 0, 0), Tup::vector(0, 0, 1));

        let s = Sphere::default()
            .with_transform(Mat::identity().scale(0.5, 0.5, 0.5))
            .with_material(Material::default());
        let i = Intersection { t: 0.5, obj: &s };

        let comps = Comps::prepare(&i, &r);
        let c = shade_hit(&w, &comps);
        assert_relative_eq!(
            c,
            Color::new(0.9049844720832575, 0.9049844720832575, 0.9049844720832575)
        );
    }
}
