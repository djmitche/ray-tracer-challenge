use crate::{spaces, Color, PatternFn, Point};

#[derive(Debug)]
pub struct Gradient {
    a: Color,
    b_minus_a: Color,
}

impl Gradient {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b_minus_a: b - a,
        }
    }
}

impl PatternFn for Gradient {
    fn color_at(&self, p: Point<spaces::Pattern>) -> Color {
        self.a + (self.b_minus_a) * p.x.fract()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;

    #[test]
    fn gradient_linear_interp() {
        let p = Gradient::new(Color::white(), Color::black());
        assert_relative_eq!(p.color_at(Point::new(0, 0, 0)), Color::white());
        assert_relative_eq!(
            p.color_at(Point::new(0.25, 0, 0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_relative_eq!(p.color_at(Point::new(0.5, 0, 0)), Color::new(0.5, 0.5, 0.5));
        assert_relative_eq!(
            p.color_at(Point::new(0.75, 0, 0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
