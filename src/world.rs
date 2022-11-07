use crate::{spaces, Color, Intersection, Intersections, Material, Object, Point, Ray, Vector};

/// An index into the objects in a world.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectIndex(usize);

impl ObjectIndex {
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
        use crate::{Mat, Sphere};
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
        let reflectv = ray.direction.reflect(normalv);
        (point, eyev, normalv, reflectv, color)
    }

    /// Calculate the lighting for the given args.
    fn lighting(
        light: &Light,
        color: Color,
        material: &Material,
        point: Point<spaces::World>,
        eyev: Vector<spaces::World>,
        normalv: Vector<spaces::World>,
        in_shadow: bool,
    ) -> Color {
        // combine surface color and light color
        let eff_color = color * light.intensity;

        // get the direction from the point to the light source
        let lightv = (light.position - point).normalize();

        // compute the ambient contribution
        let ambient = eff_color * material.ambient;

        if in_shadow {
            return ambient;
        }

        // calculate diffuse and specular
        let diffuse;
        let specular;

        // light_dot_normal is the cosine of the angle between the light vector and the normal
        // vector.  A negative number means it is on the other side of the surface.
        let light_dot_normal = lightv.dot(normalv);
        if light_dot_normal < 0.0 {
            diffuse = Color::black();
            specular = Color::black();
        } else {
            // compute the diffuse contribution
            diffuse = eff_color * material.diffuse * light_dot_normal;

            // reflect_dot_eye is the cosine of the angle between the reflection vector and the eye
            // vector.
            let reflect_dot_eye = (-lightv).reflect(normalv).dot(eyev);
            if reflect_dot_eye < 0.0 {
                specular = Color::black();
            } else {
                let factor = reflect_dot_eye.powf(material.shininess);
                specular = light.intensity * material.specular * factor;
            }
        }

        ambient + diffuse + specular
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

    fn reflected_color(
        &self,
        point: Point<spaces::World>,
        reflectv: Vector<spaces::World>,
        material: &Material,
        mut total_reflectivity: f64,
    ) -> Color {
        total_reflectivity *= material.reflectivity;
        if total_reflectivity < 0.00001 {
            return Color::black();
        }

        // move 0.01 along the ray to escape the object on which point
        // is situated
        let refl_ray = Ray::new(point + reflectv * 0.01, reflectv);
        self.color_at_inner(&refl_ray, total_reflectivity) * material.reflectivity
    }

    fn color_at_inner(&self, ray: &Ray<spaces::World>, total_reflectivity: f64) -> Color {
        let mut inters = Intersections::default();
        self.intersect(ray, &mut inters);
        if let Some(hit) = inters.hit() {
            let obj = &self.objects[hit.object_index.0];
            let (point, eyev, normalv, reflectv, color) = Self::precompute(obj, hit, ray);
            let surface = Self::lighting(
                &self.light,
                color,
                &obj.material,
                point,
                eyev,
                normalv,
                self.point_is_shadowed(point),
            );
            let reflected =
                self.reflected_color(point, reflectv, &obj.material, total_reflectivity);
            surface + reflected
        } else {
            Color::black()
        }
    }

    /// Determine the color received by an eye at the origin of the given ray.
    pub fn color_at(&self, ray: &Ray<spaces::World>) -> Color {
        self.color_at_inner(ray, 1.0)
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

        let (point, eyev, normalv, reflectv, _) = World::precompute(&shape, &i, &r);
        assert_relative_eq!(point, Point::new(0, 0, -1));
        assert_relative_eq!(eyev, Vector::new(0, 0, -1));
        assert_relative_eq!(normalv, Vector::new(0, 0, -1));
        assert_relative_eq!(reflectv, Vector::new(0, 0, -1));
    }

    #[test]
    fn precompute_state_inside() {
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let shape = Object::new(Sphere);
        let i = Intersection {
            object_index: ObjectIndex::test_value(0),
            t: 1.0,
        };

        let (point, eyev, normalv, _, _) = World::precompute(&shape, &i, &r);
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

    #[test]
    fn no_reflection() {
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
                .with_material(Material::default().with_ambient(1.0)),
        );
        assert_relative_eq!(
            w.reflected_color(
                Point::new(0, 0, 0),
                Vector::new(1, 0, 0),
                &Material::default().with_reflectivity(0.0),
                1.0
            ),
            Color::black()
        );
    }

    #[test]
    fn reflective_material() {
        let mut w = World::test_world();
        w.add_object(
            Object::new(Plane)
                .with_transform(Mat::identity().translate(0, -1, 0))
                .with_material(Material::default().with_reflectivity(0.5)),
        );
        let sqrt2over2 = 2f64.sqrt() / 2.0;
        let r = Ray::new(
            Point::new(0, 0, -3),
            Vector::new(0, -sqrt2over2, sqrt2over2),
        );
        let mut inters = Intersections::default();
        w.intersect(&r, &mut inters);
        let hit = inters.hit().unwrap();
        let obj = &w.objects[hit.object_index.0];
        let (point, _, _, reflectv, _) = World::precompute(obj, hit, &r);
        let color = w.reflected_color(point, reflectv, &obj.material, 1.0);
        assert_relative_eq!(
            color,
            Color::new(
                0.19033059654051723,
                0.23791324567564653,
                0.14274794740538793
            )
        );
    }

    #[test]
    fn eye_between_light_and_surface() {
        let m = Material::default();
        let position = Point::new(0, 0, 0);
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = Light::new_point(Point::new(0, 0, -10), Color::white());
        assert_relative_eq!(
            World::lighting(&light, Color::white(), &m, position, eyev, normalv, false),
            Color::new(1.9, 1.9, 1.9)
        );
    }

    #[test]
    fn eye_45_to_normal() {
        let m = Material::default();
        let position = Point::new(0, 0, 0);
        let eyev = Vector::new(0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let normalv = Vector::new(0, 0, -1);
        let light = Light::new_point(Point::new(0, 0, -10), Color::white());
        assert_relative_eq!(
            World::lighting(&light, Color::white(), &m, position, eyev, normalv, false),
            Color::new(1.0, 1.0, 1.0)
        );
    }

    #[test]
    fn light_45_to_normal() {
        let m = Material::default();
        let position = Point::new(0, 0, 0);
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = Light::new_point(Point::new(0, 10, -10), Color::white());
        assert_relative_eq!(
            World::lighting(&light, Color::white(), &m, position, eyev, normalv, false),
            Color::new(0.7363961030678927, 0.7363961030678927, 0.7363961030678927)
        );
    }

    #[test]
    fn light_eye_in_path_of_reflection() {
        let m = Material::default();
        let position = Point::new(0, 0, 0);
        let eyev = Vector::new(0, -2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let normalv = Vector::new(0, 0, -1);
        let light = Light::new_point(Point::new(0, 10, -10), Color::white());
        assert_relative_eq!(
            World::lighting(&light, Color::white(), &m, position, eyev, normalv, false),
            Color::new(1.6363961030678928, 1.6363961030678928, 1.6363961030678928)
        );
    }

    #[test]
    fn light_behind_surface() {
        let m = Material::default();
        let position = Point::new(0, 0, 0);
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = Light::new_point(Point::new(0, 0, 10), Color::white());
        assert_relative_eq!(
            World::lighting(&light, Color::white(), &m, position, eyev, normalv, false),
            Color::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn surface_in_shadow() {
        let m = Material::default();
        let position = Point::new(0, 0, 0);
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = Light::new_point(Point::new(0, 0, -10), Color::white());
        assert_relative_eq!(
            World::lighting(&light, Color::white(), &m, position, eyev, normalv, true),
            Color::new(0.1, 0.1, 0.1),
        );
    }
}
