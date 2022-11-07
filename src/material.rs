use crate::{Color, Pattern};

/// Material defines the relevant characteristics of a material.
#[derive(Debug, Clone)]
pub struct Material {
    pub pattern: Pattern,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflectivity: f64,
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
}
