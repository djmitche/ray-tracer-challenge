use crate::{spaces, Color, Intersection, Intersections, Light, Object, Point, Ray, Vector};

/// An index into the objects in a world.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectIndex(usize);

impl ObjectIndex {
    pub fn test_value(i: usize) -> ObjectIndex {
        ObjectIndex(i)
    }
}

/// World describes an entire world to be rendered.
#[derive(Debug)]
pub struct World {
    light: Light,
    objects: Vec<Object>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            light: Light::new_point(Point::new(-10, 10, -10), Color::white()),
            objects: vec![],
        }
    }
}

impl World {
    pub fn new(light: Light) -> Self {
        Self {
            light,
            objects: vec![],
        }
    }

    /// Add a new object to this world, returning its ObjectIndex.
    pub fn add_object(&mut self, obj: Object) -> ObjectIndex {
        let idx = ObjectIndex(self.objects.len());
        self.objects.push(obj);
        idx
    }

    pub fn set_light(&mut self, light: Light) {
        self.light = light;
    }

    /// Create the "default_world" from the tests.
    #[cfg(test)]
    pub(crate) fn test_world() -> Self {
        use crate::{Mat, Material, Sphere};
        let mut w = World::default();
        w.add_object(Object::new(Sphere).with_material(Material {
            pattern: Color::new(0.8, 1.0, 0.6).into(),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        }));
        w.add_object(
            Object::new(Sphere)
                .with_transform(Mat::identity().scale(0.5, 0.5, 0.5))
                .with_material(Material::default()),
        );
        w
    }

    /// Intersect the given ray with all objects in the world.
    fn intersect(&self, ray: &Ray<spaces::World>, inters: &mut Intersections) {
        for (i, o) in self.objects.iter().enumerate() {
            o.intersect(ObjectIndex(i), ray, inters);
        }
    }

    /// Precompute the point of intersection, the eye vector, and the normal vector
    /// for the given hit of the given ray.
    fn precompute(
        obj: &Object,
        hit: &Intersection,
        ray: &Ray<spaces::World>,
    ) -> (
        Point<spaces::World>,
        Vector<spaces::World>,
        Vector<spaces::World>,
        Color,
    ) {
        let point = ray.position(hit.t);
        let eyev = -ray.direction;
        let (mut normalv, color) = obj.normal_and_color(point);
        if normalv.dot(eyev) < 0.0 {
            // use the inside surface, with the opposite normal
            normalv = -normalv;
        }
        (point, eyev, normalv, color)
    }

    fn point_is_shadowed(&self, point: Point<spaces::World>) -> bool {
        let to_light = self.light.position - point;
        let to_light_norm = to_light.normalize();
        // move 0.01 along the ray to escape the object on which point
        // is situated
        let to_light_ray = Ray::new(point + to_light_norm * 0.01, to_light_norm);

        let mut inters = Intersections::default();
        self.intersect(&to_light_ray, &mut inters);
        if let Some(hit) = inters.hit() {
            hit.t < to_light.magnitude()
        } else {
            false
        }
    }

    /// Determine the color received by an eye at the origin of the given ray.
    pub fn color_at(&self, ray: &Ray<spaces::World>) -> Color {
        let mut inters = Intersections::default();
        self.intersect(ray, &mut inters);
        if let Some(hit) = inters.hit() {
            let obj = &self.objects[hit.object_index.0];
            let (point, eyev, normalv, color) = Self::precompute(obj, hit, ray);
            self.light.lighting(
                color,
                &obj.material,
                point,
                eyev,
                normalv,
                self.point_is_shadowed(point),
            )
        } else {
            Color::black()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;
    use approx::*;

    #[test]
    fn precompute_state() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let shape = Object::new(Sphere);
        let i = Intersection {
            object_index: ObjectIndex::test_value(0),
            t: 4.0,
        };

        let (point, eyev, normalv, _) = World::precompute(&shape, &i, &r);
        assert_relative_eq!(point, Point::new(0, 0, -1));
        assert_relative_eq!(eyev, Vector::new(0, 0, -1));
        assert_relative_eq!(normalv, Vector::new(0, 0, -1));
    }

    #[test]
    fn precompute_state_inside() {
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let shape = Object::new(Sphere);
        let i = Intersection {
            object_index: ObjectIndex::test_value(0),
            t: 1.0,
        };

        let (point, eyev, normalv, _) = World::precompute(&shape, &i, &r);
        assert_relative_eq!(point, Point::new(0, 0, 1));
        assert_relative_eq!(eyev, Vector::new(0, 0, -1));
        assert_relative_eq!(normalv, Vector::new(0, 0, -1));
    }

    #[test]
    fn shade_intersection_inside() {
        let mut w = World::test_world();
        w.light = Light::new_point(Point::new(0, 0.25, 0), Color::white());
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let c = w.color_at(&r);
        assert_relative_eq!(
            c,
            Color::new(0.9049844720832575, 0.9049844720832575, 0.9049844720832575)
        );
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = World::test_world();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let mut inters = Intersections::default();
        w.intersect(&r, &mut inters);
        let mut it = inters.iter();
        assert_relative_eq!(it.next().expect("intersection").t, 4.0);
        assert_relative_eq!(it.next().expect("intersection").t, 4.5);
        assert_relative_eq!(it.next().expect("intersection").t, 5.5);
        assert_relative_eq!(it.next().expect("intersection").t, 6.0);
        assert!(it.next().is_none());
    }

    #[test]
    fn color_at_miss() {
        let w = World::test_world();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 1, 0));
        assert_relative_eq!(w.color_at(&r), Color::black());
    }

    #[test]
    fn color_at_hit() {
        let w = World::test_world();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        assert_relative_eq!(
            w.color_at(&r),
            Color::new(
                0.38066119308103435,
                0.47582649135129296,
                0.28549589481077575
            )
        );
    }

    #[test]
    fn color_at_behind_ray() {
        let mut w = World::default();
        w.add_object(Object::new(Sphere).with_material(Material {
            pattern: Color::new(0.8, 1.0, 0.6).into(),
            diffuse: 0.7,
            specular: 0.2,
            ambient: 1.0,
            ..Default::default()
        }));
        w.add_object(
            Object::new(Sphere)
                .with_transform(Mat::identity().scale(0.5, 0.5, 0.5))
                .with_material(Material {
                    ambient: 1.0,
                    ..Default::default()
                }),
        );
        let r = Ray::new(Point::new(0, 0, 0.75), Vector::new(0, 0, -1));
        assert_relative_eq!(w.color_at(&r), Color::white());
    }

    #[test]
    fn no_shadow_when_nothing_collinear() {
        let w = World::test_world();
        let p = Point::new(0, 0, -5);
        assert!(!w.point_is_shadowed(p));
    }

    #[test]
    fn shadow_when_obj_intervenes() {
        let w = World::test_world();
        let p = Point::new(10, -10, 10);
        assert!(w.point_is_shadowed(p));
    }

    #[test]
    fn no_shadow_when_obj_behind_light() {
        let w = World::test_world();
        let p = Point::new(-20, 20, -20);
        assert!(!w.point_is_shadowed(p));
    }

    #[test]
    fn no_shadow_when_obj_behind_point() {
        let w = World::test_world();
        let p = Point::new(-2, 2, -2);
        assert!(!w.point_is_shadowed(p));
    }
}
