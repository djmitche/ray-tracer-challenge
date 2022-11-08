use crate::{spaces, Color, Pattern, Point, Ray, Vector, World};

/// Material defines the relevant characteristics of a material.
#[derive(Debug, Clone)]
pub struct Material {
    pattern: Pattern,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    reflectivity: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            pattern: Color::new(1, 1, 1).into(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflectivity: 0.0,
        }
    }
}

impl Material {
    pub fn with_color(mut self, color: Color) -> Self {
        self.pattern = color.into();
        self
    }

    pub fn with_pattern(mut self, pattern: Pattern) -> Self {
        self.pattern = pattern;
        self
    }

    pub fn with_ambient(mut self, ambient: f64) -> Self {
        self.ambient = ambient;
        self
    }

    pub fn with_diffuse(mut self, diffuse: f64) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn with_specular(mut self, specular: f64) -> Self {
        self.specular = specular;
        self
    }

    pub fn with_shininess(mut self, shininess: f64) -> Self {
        self.shininess = shininess;
        self
    }

    pub fn with_reflectivity(mut self, reflectivity: f64) -> Self {
        self.reflectivity = reflectivity;
        self
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
        world.color_at(&refl_ray, total_contribution * self.reflectivity) * self.reflectivity
    }

    pub(crate) fn color_at(
        &self,
        world: &World,
        ray: &Ray<spaces::World>,
        world_point: Point<spaces::World>,
        obj_point: Point<spaces::Object>,
        eyev: Vector<spaces::World>,
        normalv: Vector<spaces::World>,
        total_contribution: f64,
    ) -> Color {
        let material_color = self.pattern.color_at(obj_point);

        let light_at = world.light_at(world_point);
        // combine material color and light color
        let eff_color = material_color * light_at.intensity;

        // compute the ambient contribution
        let mut color = eff_color * self.ambient;

        // diffuse and specular only appear if not in shadow
        if !light_at.in_shadow {
            // light_dot_normal is the cosine of the angle between the light vector and the normal
            // vector.  A negative number means it is on the other side of the surface.
            let light_dot_normal = light_at.direction.dot(normalv);

            // calculate diffuse and specular
            if light_dot_normal > 0.0 {
                // compute the diffuse contribution
                color = color + eff_color * self.diffuse * light_dot_normal;

                // reflect_dot_eye is the cosine of the angle between the reflection vector and the eye
                // vector.
                let reflect_dot_eye = (-light_at.direction).reflect(normalv).dot(eyev);
                if reflect_dot_eye > 0.0 {
                    let factor = reflect_dot_eye.powf(self.shininess);
                    color = color + light_at.intensity * self.specular * factor;
                }
            }
        }

        // add reflected color
        if self.reflectivity > 0.0 {
            let reflectv = ray.direction.reflect(normalv);
            color = color + self.reflected_color(world, world_point, reflectv, total_contribution);
        };

        color
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use approx::*;
    use std::f64::consts::PI;

    #[test]
    fn eye_between_light_and_surface() {
        let position = Point::new(0, 0, 0);
        let eyev = Vector::new(0, 0, -1);
        let ray = Ray::new(position, -eyev);
        let normalv = Vector::new(0, 0, -1);
        let w = World::new(Light::new_point(Point::new(0, 0, -10), Color::white()));
        let m = Material::default();
        assert_relative_eq!(
            m.color_at(&w, &ray, position, position.as_space(), eyev, normalv, 1.0),
            Color::new(1.9, 1.9, 1.9)
        );
    }

    #[test]
    fn eye_45_to_normal() {
        let position = Point::new(0, 0, 0);
        let eyev = Vector::new(0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let ray = Ray::new(position, -eyev);
        let normalv = Vector::new(0, 0, -1);
        let w = World::new(Light::new_point(Point::new(0, 0, -10), Color::white()));
        let m = Material::default();
        assert_relative_eq!(
            m.color_at(&w, &ray, position, position.as_space(), eyev, normalv, 1.0),
            Color::new(1.0, 1.0, 1.0)
        );
    }

    #[test]
    fn light_45_to_normal() {
        let position = Point::new(0, 0, 0);
        let eyev = Vector::new(0, 0, -1);
        let ray = Ray::new(position, -eyev);
        let normalv = Vector::new(0, 0, -1);
        let w = World::new(Light::new_point(Point::new(0, 10, -10), Color::white()));
        let m = Material::default();
        assert_relative_eq!(
            m.color_at(&w, &ray, position, position.as_space(), eyev, normalv, 1.0),
            Color::new(0.7363961030678927, 0.7363961030678927, 0.7363961030678927)
        );
    }

    #[test]
    fn light_eye_in_path_of_reflection() {
        let position = Point::new(0, 0, 0);
        let eyev = Vector::new(0, -2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let ray = Ray::new(position, -eyev);
        let normalv = Vector::new(0, 0, -1);
        let w = World::new(Light::new_point(Point::new(0, 10, -10), Color::white()));
        let m = Material::default();
        assert_relative_eq!(
            m.color_at(&w, &ray, position, position.as_space(), eyev, normalv, 1.0),
            Color::new(1.6363961030678928, 1.6363961030678928, 1.6363961030678928)
        );
    }

    #[test]
    fn light_behind_surface() {
        let position = Point::new(0, 0, 0);
        let eyev = Vector::new(0, 0, -1);
        let ray = Ray::new(position, -eyev);
        let normalv = Vector::new(0, 0, -1);
        let w = World::new(Light::new_point(Point::new(0, 0, 10), Color::white()));
        let m = Material::default();
        assert_relative_eq!(
            m.color_at(&w, &ray, position, position.as_space(), eyev, normalv, 1.0),
            Color::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn surface_in_shadow() {
        let position = Point::new(0, 0, 0);
        let eyev = Vector::new(0, 0, -1);
        let ray = Ray::new(position, -eyev);
        let normalv = Vector::new(0, 0, -1);
        let mut w = World::new(Light::new_point(Point::new(0, 0, -10), Color::white()));
        // add a plane between light and position, to shadow it
        w.add_object(
            Object::new(Plane)
                .with_transform(Mat::identity().rotate_x(PI / 2.0).translate(0, 0, -5)),
        );
        let m = Material::default();
        assert_relative_eq!(
            m.color_at(&w, &ray, position, position.as_space(), eyev, normalv, 1.0),
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
        let m = Material::default().with_reflectivity(0.0);
        assert_relative_eq!(
            m.reflected_color(&w, Point::new(0, 0, 0), Vector::new(1, 0, 0), 1.0),
            Color::black()
        );
    }

    #[test]
    fn mirror_reflection() {
        let mut w = World::default();
        // non-reflective blue plane at y = 2
        w.add_object(
            Object::new(Plane)
                .with_transform(Mat::identity().translate(0, 2, 0))
                .with_material(
                    Material::default()
                        .with_color(Color::new(0, 0, 1.0))
                        .with_ambient(1.0),
                ),
        );
        // make believe we have a reflection off a plane at y = 0
        let m = Material::default().with_reflectivity(1.0);
        assert_relative_eq!(
            m.reflected_color(&w, Point::new(0, 0, 0), Vector::new(0, 1, 0), 1.0),
            Color::new(0, 0, 1),
        );
    }
}
