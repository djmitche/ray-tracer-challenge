use crate::{spaces, Color, PatternFn, Point};

#[derive(Debug)]
pub struct Checker(Color, Color);

impl Checker {
    pub fn new(a: Color, b: Color) -> Self {
        Self(a, b)
    }
}

impl PatternFn for Checker {
    fn color_at(&self, p: Point<spaces::Pattern>) -> Color {
        if (p.x.floor() + p.y.floor() + p.z.floor()).rem_euclid(2.0) < 1.0 {
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
    fn checker_repeat_x() {
        let p = Checker::new(Color::white(), Color::black());
        assert_relative_eq!(p.color_at(Point::new(0, 0, 0)), Color::white());
        assert_relative_eq!(p.color_at(Point::new(0.99, 0, 0)), Color::white());
        assert_relative_eq!(p.color_at(Point::new(1.01, 0, 0)), Color::black());
    }

    #[test]
    fn checker_repeat_y() {
        let p = Checker::new(Color::white(), Color::black());
        assert_relative_eq!(p.color_at(Point::new(0, 0, 0)), Color::white());
        assert_relative_eq!(p.color_at(Point::new(0, 0.99, 0)), Color::white());
        assert_relative_eq!(p.color_at(Point::new(0, 1.01, 0)), Color::black());
    }

    #[test]
    fn checker_repeat_z() {
        let p = Checker::new(Color::white(), Color::black());
        assert_relative_eq!(p.color_at(Point::new(0, 0, 0)), Color::white());
        assert_relative_eq!(p.color_at(Point::new(0, 0, 0.99)), Color::white());
        assert_relative_eq!(p.color_at(Point::new(0, 0, 1.01)), Color::black());
    }
}
