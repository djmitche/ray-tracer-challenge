#![allow(dead_code)]
#![allow(unused_imports)]
use approx::{relative_eq, AbsDiffEq, RelativeEq};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new<R: Into<f64>, G: Into<f64>, B: Into<f64>>(red: R, green: G, blue: B) -> Self {
        Self {
            red: red.into(),
            green: green.into(),
            blue: blue.into(),
        }
    }

    pub fn black() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    pub fn white() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }

    /// Iterate over the elements of the color in RGB order.
    pub fn iter(&self) -> ColorIterator<'_> {
        ColorIterator(self, 0)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }
}

impl AbsDiffEq for Color {
    type Epsilon = <f64 as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> <f64 as AbsDiffEq>::Epsilon {
        <f64 as AbsDiffEq>::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: <f64 as AbsDiffEq>::Epsilon) -> bool {
        <f64 as AbsDiffEq>::abs_diff_eq(&self.red, &other.red, epsilon)
            && <f64 as AbsDiffEq>::abs_diff_eq(&self.green, &other.green, epsilon)
            && <f64 as AbsDiffEq>::abs_diff_eq(&self.blue, &other.blue, epsilon)
    }
}

impl RelativeEq for Color {
    fn default_max_relative() -> <f64 as AbsDiffEq>::Epsilon {
        <f64 as RelativeEq>::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: <f64 as AbsDiffEq>::Epsilon,
        max_relative: <f64 as AbsDiffEq>::Epsilon,
    ) -> bool {
        <f64 as RelativeEq>::relative_eq(&self.red, &other.red, epsilon, max_relative)
            && <f64 as RelativeEq>::relative_eq(&self.green, &other.green, epsilon, max_relative)
            && <f64 as RelativeEq>::relative_eq(&self.blue, &other.blue, epsilon, max_relative)
    }
}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl std::ops::Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            red: self.red * other,
            green: self.green * other,
            blue: self.blue * other,
        }
    }
}

impl std::ops::Div<f64> for Color {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            red: self.red / other,
            green: self.green / other,
            blue: self.blue / other,
        }
    }
}

impl std::ops::Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

pub struct ColorIterator<'a>(&'a Color, u8);

impl Iterator for ColorIterator<'_> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let v = match self.1 {
            0 => self.0.red,
            1 => self.0.green,
            2 => self.0.blue,
            _ => return None,
        };
        self.1 += 1;
        Some(v)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_relative_eq;

    /// Colors are (red, green, blue) tuples
    #[test]
    fn colors_are_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_relative_eq!(c.red, -0.5);
    }

    /// Adding colors
    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_relative_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    /// Subtracting colors
    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_relative_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    /// Multiplying a color by a scalar
    #[test]
    fn mult_colors_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_relative_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    /// Multiplying a color by a color
    #[test]
    fn mult_colors() {
        let c1 = Color::new(1, 0.2, 0.4);
        let c2 = Color::new(0.9, 1, 0.1);
        assert_relative_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }
}
