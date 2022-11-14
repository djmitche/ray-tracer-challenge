use crate::{spaces, Color, Intersections, Object, Point, Ray, RayColor, Vector};

/// The minimum total_contribution for which color_at_inner will make a calculation
const MIN_CONTRIBUTION: f64 = 0.001;

/// An index into the objects in a world.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectIndex(usize);

impl ObjectIndex {
    #[cfg(test)]
    pub fn test_value(i: usize) -> ObjectIndex {
        ObjectIndex(i)
    }
}

/// A representation of a light in the world.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Light {
    pub position: Point<spaces::World>,
    pub intensity: Color,
}

impl Light {
    pub fn new_point(position: Point<spaces::World>, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

/// A representation of light's effect at a particular point
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub(crate) struct LightAt {
    /// The intensity of the light
    pub(crate) intensity: Color,

    /// Direction from the given point to the light
    pub(crate) direction: Vector<spaces::World>,

    /// Whether the point is in shadow
    pub(crate) in_shadow: bool,
}

/// World describes an entire world to be rendered.
#[derive(Debug)]
pub struct World {
    pub(crate) light: Light,
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
        w.add_object(
            Object::new(Sphere).with_material(
                Material::default()
                    .with_color(Color::new(0.8, 1.0, 0.6))
                    .with_diffuse(0.7)
                    .with_specular(0.2),
            ),
        );
        w.add_object(
            Object::new(Sphere)
                .with_transform(Mat::identity().scale(0.5, 0.5, 0.5))
                .with_material(Material::default()),
        );
        w
    }

    /// Intersect the given ray with all objects in the world.
    pub(crate) fn intersect(&self, ray: &Ray<spaces::World>, inters: &mut Intersections) {
        for (i, o) in self.objects.iter().enumerate() {
            o.intersect(ObjectIndex(i), ray, inters);
        }
    }

    fn point_is_shadowed(&self, point: Point<spaces::World>) -> bool {
        let to_light = self.light.position - point;
        let to_light_norm = to_light.normalize();
        // move 0.01 along the ray to escape the object on which point
        // is situated
        let to_light_ray = Ray::new(point + to_light_norm * 0.01, to_light_norm);

        let mut inters = Intersections::default();
        self.intersect(&to_light_ray, &mut inters);
        if let (_, Some(t), _) = inters.hit() {
            t < to_light.magnitude()
        } else {
            false
        }
    }

    /// Calculate the effect of the world's light at the given point.
    pub(crate) fn light_at(&self, point: Point<spaces::World>) -> LightAt {
        LightAt {
            intensity: self.light.intensity,
            direction: (self.light.position - point).normalize(),
            in_shadow: self.point_is_shadowed(point),
        }
    }

    /// Determine the color received by an eye at the origin of the given ray, with
    /// a measure of total contribution to the final pixel.
    ///
    /// This function may recurse, and will terminate when the total contribution is small enough
    /// to not matter.  The initial `total_contribution` should be zero.
    pub(crate) fn color_at(
        &self,
        ray: &Ray<spaces::World>,
        total_contribution: f64,
        debug: bool,
    ) -> Color {
        if debug {
            dbg!((ray, total_contribution));
        }
        // stop recursing when the effect is small enough
        if total_contribution < MIN_CONTRIBUTION {
            return Color::black();
        }

        let mut inters = Intersections::default();
        self.intersect(ray, &mut inters);
        if debug {
            dbg!(&inters);
        }
        if let (from_obj_idx, Some(t), to_obj_idx) = inters.hit() {
            if debug {
                dbg!((from_obj_idx, Some(t), to_obj_idx));
            }
            let from_obj = from_obj_idx.map(|i| &self.objects[i.0]);
            let to_obj = to_obj_idx.map(|i| &self.objects[i.0]);
            // one of from_obj or to_obj must exist, since we got a `t` value
            let hit_obj = to_obj.or(from_obj).expect("should have had an object");
            hit_obj.color_at(self, from_obj, t, ray, total_contribution, debug)
        } else {
            if debug {
                println!("no hits");
            }
            Color::black()
        }
    }
}

impl RayColor for World {
    fn color_at(&self, ray: &Ray<spaces::World>, debug: bool) -> Color {
        World::color_at(self, ray, 1.0, debug)
    }
}

/// Index the world by ObjectIndex to get an object reference.  Note that
/// ObjectIndex instances are only created when objects are added, so the
/// object is guaranteed to exist.
impl std::ops::Index<ObjectIndex> for World {
    type Output = Object;

    fn index(&self, idx: ObjectIndex) -> &Object {
        &self.objects[idx.0]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;
    use approx::*;

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
        assert_relative_eq!(w.color_at(&r, 1.0, true), Color::black());
    }

    #[test]
    fn color_at_hit() {
        let w = World::test_world();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        assert_relative_eq!(
            w.color_at(&r, 1.0, true),
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
        w.add_object(
            Object::new(Sphere).with_material(
                Material::default()
                    .with_color(Color::new(0.8, 1.0, 0.6))
                    .with_diffuse(0.7)
                    .with_specular(0.2)
                    .with_ambient(1.0),
            ),
        );
        w.add_object(
            Object::new(Sphere)
                .with_transform(Mat::identity().scale(0.5, 0.5, 0.5))
                .with_material(Material::default().with_ambient(1.0)),
        );
        let r = Ray::new(Point::new(0, 0, 0.75), Vector::new(0, 0, -1));
        assert_relative_eq!(w.color_at(&r, 1.0, true), Color::white());
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
