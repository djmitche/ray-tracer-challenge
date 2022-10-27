use crate::{spaces, Color, PatternFn, Point};

#[derive(Debug)]
pub struct Ring(Color, Color);

impl Ring {
    pub fn new(a: Color, b: Color) -> Self {
        Self(a, b)
    }
}

impl PatternFn for Ring {
    fn color_at(&self, p: Point<spaces::Pattern>) -> Color {
        let d = (p.x * p.x + p.z * p.z).sqrt();
        if d.rem_euclid(2.0) < 1.0 {
            self.0
        } else {
            self.1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;

    #[test]
    fn ring_both_x_and_z() {
        let p = Ring::new(Color::white(), Color::black());
        assert_relative_eq!(p.color_at(Point::new(0, 0, 0)), Color::white());
        assert_relative_eq!(p.color_at(Point::new(1, 0, 0)), Color::black());
        assert_relative_eq!(p.color_at(Point::new(0, 0, 1)), Color::black());
        assert_relative_eq!(p.color_at(Point::new(0.708, 0, 0.708)), Color::black());
    }
}
