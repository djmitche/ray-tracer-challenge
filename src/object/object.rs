use crate::{
    spaces, Color, Intersection, Intersections, Mat, Material, ObjectIndex, Point, Ray, Vector,
    World,
};

/// ObjectInnner defines methods to handle the particularities of an object, in object space.
pub trait ObjectInner: std::fmt::Debug + Sync + Send {
    /// Intersect calculates the intersections of the given ray with this object.
    fn intersect(
        &self,
        object_index: ObjectIndex,
        ray: Ray<spaces::Object>,
        inters: &mut Intersections,
    );

    /// Normal calculates the normal of the given point on the surface of this object.
    fn normal(&self, point: Point<spaces::Object>) -> Vector<spaces::Object>;
}

#[derive(Debug)]
pub struct Object {
    /// The implementation of this object, in object space
    inner: Box<dyn ObjectInner>,

    /// The transformation from world to object space.
    transform: Mat<4, spaces::World, spaces::Object>,

    /// The transformation of object-space normals to world-space normals.
    transp_transform: Mat<4, spaces::Object, spaces::World>,

    /// The material comprising this object.
    pub(crate) material: Material,
}

impl Object {
    /// Create a new object.
    ///
    /// Use the `with_...` methods to adjust the transform and material, in
    /// a builder pattern.
    pub fn new(inner: impl ObjectInner + 'static) -> Self {
        Self {
            inner: Box::new(inner),
            transform: Mat::identity(),
            transp_transform: Mat::identity(),
            material: Material::default(),
        }
    }

    /// Return an updated object with the given transform.
    pub fn with_transform(mut self, obj_to_world: Mat<4, spaces::Object, spaces::World>) -> Self {
        self.transform = obj_to_world.inverse();
        self.transp_transform = self.transform.transpose();
        self
    }

    /// Return an updated object with the given material.
    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    /// Calculate the intersections of the given ray with this object.
    pub(crate) fn intersect(
        &self,
        object_index: ObjectIndex,
        ray: &Ray<spaces::World>,
        inters: &mut Intersections,
    ) {
        let obj_ray = self.transform * *ray;
        self.inner.intersect(object_index, obj_ray, inters);
    }

    /// Calculate the surface lighting for the given args.
    fn surface_color(
        &self,
        world: &World,
        material_color: Color,
        point: Point<spaces::World>,
        eyev: Vector<spaces::World>,
        normalv: Vector<spaces::World>,
    ) -> Color {
        let light_at = world.light_at(point);
        // combine material color and light color
        let eff_color = material_color * light_at.intensity;

        // compute the ambient contribution
        let ambient = eff_color * self.material.ambient;

        if light_at.in_shadow {
            return ambient;
        }

        // calculate diffuse and specular
        let diffuse;
        let specular;

        // light_dot_normal is the cosine of the angle between the light vector and the normal
        // vector.  A negative number means it is on the other side of the surface.
        let light_dot_normal = light_at.direction.dot(normalv);
        if light_dot_normal < 0.0 {
            diffuse = Color::black();
            specular = Color::black();
        } else {
            // compute the diffuse contribution
            diffuse = eff_color * self.material.diffuse * light_dot_normal;

            // reflect_dot_eye is the cosine of the angle between the reflection vector and the eye
            // vector.
            let reflect_dot_eye = (-light_at.direction).reflect(normalv).dot(eyev);
            if reflect_dot_eye < 0.0 {
                specular = Color::black();
            } else {
                let factor = reflect_dot_eye.powf(self.material.shininess);
                specular = light_at.intensity * self.material.specular * factor;
            }
        }

        ambient + diffuse + specular
    }

    fn reflected_color(
        &self,
        world: &World,
        point: Point<spaces::World>,
        reflectv: Vector<spaces::World>,
        total_contribution: f64,
    ) -> Color {
        // move 0.01 along the ray to escape the object on which point
        // is situated
        let refl_ray = Ray::new(point + reflectv * 0.01, reflectv);
        world.color_at(&refl_ray, total_contribution * self.material.reflectivity)
            * self.material.reflectivity
    }

    pub(crate) fn color_at(
        &self,
        world: &World,
        hit: &Intersection,
        ray: &Ray<spaces::World>,
        total_contribution: f64,
    ) -> Color {
        // the point at which the intersection occurred
        let point = ray.position(hit.t);

        // vector from point to the eye
        let eyev = -ray.direction;

        // point in object space
        let obj_point = self.transform * point;

        // normal in object space
        let obj_normal = self.inner.normal(obj_point);

        // normal in world space
        let mut normalv = (self.transp_transform * obj_normal).normalize();
        if normalv.dot(eyev) < 0.0 {
            // use the inside surface, with the opposite normal
            normalv = -normalv;
        }

        let material_color = self.material.pattern.color_at(obj_point);

        // calculate surface color
        let mut color = self.surface_color(world, material_color, point, eyev, normalv);

        // add reflected color
        if self.material.reflectivity > 0.0 {
            let reflectv = ray.direction.reflect(normalv);
            color = color + self.reflected_color(world, point, reflectv, total_contribution);
        };

        color
    }

    /// Get only the normal (used for testing objects)
    #[cfg(test)]
    pub fn normal(&self, point: Point<spaces::World>) -> Vector<spaces::World> {
        let obj_point = self.transform * point;
        let obj_normal = self.inner.normal(obj_point);
        let world_normal = self.transp_transform * obj_normal;
        world_normal.normalize()
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use approx::*;
    use std::f64::consts::PI;

    #[test]
    fn defaults() {
        let o = Object::new(Sphere);
        assert_relative_eq!(o.transform, Mat::identity());
        assert_relative_eq!(o.transp_transform, Mat::identity());
        assert_relative_eq!(o.material.ambient, Material::default().ambient);
    }

    #[test]
    fn with_transform() {
        let t = Mat::identity().rotate_x(1.0).rotate_y(2.0);
        let o = Object::new(Sphere).with_transform(t);
        assert_relative_eq!(o.transform, t.inverse());
        assert_relative_eq!(o.transp_transform, t.inverse().transpose());
        assert_relative_eq!(o.material.ambient, Material::default().ambient);
    }

    #[test]
    fn with_material() {
        let mat = Material::default().with_ambient(13.0);
        let o = Object::new(Sphere).with_material(mat);
        assert_relative_eq!(o.transform, Mat::identity());
        assert_relative_eq!(o.transp_transform, Mat::identity());
        assert_relative_eq!(o.material.ambient, 13.0);
    }

    #[test]
    fn with_both() {
        let t = Mat::identity().rotate_x(1.0).rotate_y(2.0);
        let o = Object::new(Sphere)
            .with_material(Material::default().with_ambient(13.0))
            .with_transform(t);
        assert_relative_eq!(o.transform, t.inverse());
        assert_relative_eq!(o.transp_transform, t.inverse().transpose());
        assert_relative_eq!(o.material.ambient, 13.0);
    }

    #[test]
    fn intersect() {
        let o = Object::new(Sphere)
            .with_transform(Mat::identity().translate(0, 0, 10))
            .with_material(Material::default().with_ambient(13.0));
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 2));
        let mut inters = Intersections::default();
        o.intersect(ObjectIndex::test_value(0), &r, &mut inters);
        // ray of length 2 hits sphere at 0, 0, 9, so t=4.5
        let hit = inters.hit().expect("intersection");
        assert_relative_eq!(hit.t, 4.5);
        assert_eq!(hit.object_index, ObjectIndex::test_value(0));
    }

    #[test]
    fn normal() {
        // a sphere stretched 2x vertically
        let o = Object::new(Sphere).with_transform(Mat::identity().scale(1, 2, 1));
        let r: Ray<spaces::World> = Ray::new(Point::new(0, 1, -10), Vector::new(0, 0, 1));

        // ray hits halfway up the sphere.
        let mut inters = Intersections::default();
        o.intersect(ObjectIndex::test_value(0), &r, &mut inters);
        let hit = inters.hit().expect("intersection");
        let p = r.position(hit.t);

        let n = o.normal(p);

        assert_relative_eq!(n.magnitude(), 1.0);
        assert_eq!(hit.object_index, ObjectIndex::test_value(0));
    }

    #[test]
    fn eye_between_light_and_surface() {
        let position = Point::new(0, 0, 0);
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let o = Object::new(Sphere);
        let w = World::new(Light::new_point(Point::new(0, 0, -10), Color::white()));
        assert_relative_eq!(
            o.surface_color(&w, Color::white(), position, eyev, normalv),
            Color::new(1.9, 1.9, 1.9)
        );
    }

    #[test]
    fn eye_45_to_normal() {
        let position = Point::new(0, 0, 0);
        let eyev = Vector::new(0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let normalv = Vector::new(0, 0, -1);
        let o = Object::new(Sphere);
        let w = World::new(Light::new_point(Point::new(0, 0, -10), Color::white()));
        assert_relative_eq!(
            o.surface_color(&w, Color::white(), position, eyev, normalv),
            Color::new(1.0, 1.0, 1.0)
        );
    }

    #[test]
    fn light_45_to_normal() {
        let position = Point::new(0, 0, 0);
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let o = Object::new(Sphere);
        let w = World::new(Light::new_point(Point::new(0, 10, -10), Color::white()));
        assert_relative_eq!(
            o.surface_color(&w, Color::white(), position, eyev, normalv),
            Color::new(0.7363961030678927, 0.7363961030678927, 0.7363961030678927)
        );
    }

    #[test]
    fn light_eye_in_path_of_reflection() {
        let position = Point::new(0, 0, 0);
        let eyev = Vector::new(0, -2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let normalv = Vector::new(0, 0, -1);
        let o = Object::new(Sphere);
        let w = World::new(Light::new_point(Point::new(0, 10, -10), Color::white()));
        assert_relative_eq!(
            o.surface_color(&w, Color::white(), position, eyev, normalv),
            Color::new(1.6363961030678928, 1.6363961030678928, 1.6363961030678928)
        );
    }

    #[test]
    fn light_behind_surface() {
        let position = Point::new(0, 0, 0);
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let o = Object::new(Sphere);
        let w = World::new(Light::new_point(Point::new(0, 0, 10), Color::white()));
        assert_relative_eq!(
            o.surface_color(&w, Color::white(), position, eyev, normalv),
            Color::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn surface_in_shadow() {
        let position = Point::new(0, 0, 0);
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let o = Object::new(Sphere);
        let mut w = World::new(Light::new_point(Point::new(0, 0, -10), Color::white()));
        // add a plane between light and position, to shadow it
        w.add_object(
            Object::new(Plane)
                .with_transform(Mat::identity().rotate_x(PI / 2.0).translate(0, 0, -5)),
        );
        assert_relative_eq!(
            o.surface_color(&w, Color::white(), position, eyev, normalv),
            Color::new(0.1, 0.1, 0.1),
        );
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
        let o = Object::new(Sphere).with_material(Material::default().with_reflectivity(0.0));
        assert_relative_eq!(
            o.reflected_color(&w, Point::new(0, 0, 0), Vector::new(1, 0, 0), 1.0),
            Color::black()
        );
    }

    /*
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
        let point: Point<spaces::World> = Point::new(0, -1, -2);
        let reflectv: Vector<spaces::World> =
            Vector::new(0, 0.7071067811865476, 0.7071067811865476);

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
    */
}
