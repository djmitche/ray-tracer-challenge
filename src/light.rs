use crate::{spaces, Color, Material, Tup};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Light {
    pub position: Tup<spaces::World>,
    pub intensity: Color,
}

impl Light {
    pub fn new_point(position: Tup<spaces::World>, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }

    /// Calculate the lighting for the given args.
    pub fn lighting(
        &self,
        material: &Material,
        point: Tup<spaces::World>,
        eyev: Tup<spaces::World>,
        normalv: Tup<spaces::World>,
        in_shadow: bool,
    ) -> Color {
        // combine surface color and light color
        let eff_color = material.color * self.intensity;

        // get the direction from the point to the light source
        let lightv = (self.position - point).normalize();

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
                specular = self.intensity * material.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;

    #[test]
    fn eye_between_light_and_surface() {
        let m = Material::default();
        let position = Tup::point(0, 0, 0);
        let eyev = Tup::vector(0, 0, -1);
        let normalv = Tup::vector(0, 0, -1);
        let light = Light::new_point(Tup::point(0, 0, -10), Color::white());
        assert_relative_eq!(
            light.lighting(&m, position, eyev, normalv, false),
            Color::new(1.9, 1.9, 1.9)
        );
    }

    #[test]
    fn eye_45_to_normal() {
        let m = Material::default();
        let position = Tup::point(0, 0, 0);
        let eyev = Tup::vector(0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let normalv = Tup::vector(0, 0, -1);
        let light = Light::new_point(Tup::point(0, 0, -10), Color::white());
        assert_relative_eq!(
            light.lighting(&m, position, eyev, normalv, false),
            Color::new(1.0, 1.0, 1.0)
        );
    }

    #[test]
    fn light_45_to_normal() {
        let m = Material::default();
        let position = Tup::point(0, 0, 0);
        let eyev = Tup::vector(0, 0, -1);
        let normalv = Tup::vector(0, 0, -1);
        let light = Light::new_point(Tup::point(0, 10, -10), Color::white());
        assert_relative_eq!(
            light.lighting(&m, position, eyev, normalv, false),
            Color::new(0.7363961030678927, 0.7363961030678927, 0.7363961030678927)
        );
    }

    #[test]
    fn light_eye_in_path_of_reflection() {
        let m = Material::default();
        let position = Tup::point(0, 0, 0);
        let eyev = Tup::vector(0, -2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let normalv = Tup::vector(0, 0, -1);
        let light = Light::new_point(Tup::point(0, 10, -10), Color::white());
        assert_relative_eq!(
            light.lighting(&m, position, eyev, normalv, false),
            Color::new(1.6363961030678928, 1.6363961030678928, 1.6363961030678928)
        );
    }

    #[test]
    fn light_behind_surface() {
        let m = Material::default();
        let position = Tup::point(0, 0, 0);
        let eyev = Tup::vector(0, 0, -1);
        let normalv = Tup::vector(0, 0, -1);
        let light = Light::new_point(Tup::point(0, 0, 10), Color::white());
        assert_relative_eq!(
            light.lighting(&m, position, eyev, normalv, false),
            Color::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn surface_in_shadow() {
        let m = Material::default();
        let position = Tup::point(0, 0, 0);
        let eyev = Tup::vector(0, 0, -1);
        let normalv = Tup::vector(0, 0, -1);
        let light = Light::new_point(Tup::point(0, 0, -10), Color::white());
        assert_relative_eq!(
            light.lighting(&m, position, eyev, normalv, true),
            Color::new(0.1, 0.1, 0.1),
        );
    }
}
