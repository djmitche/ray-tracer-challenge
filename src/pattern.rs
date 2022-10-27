use crate::{spaces, Color, Mat, Point};

/// Pattern defines a pattern of colors in object space
#[derive(Debug)]
pub struct Pattern {
    transform: Mat<4, spaces::Object, spaces::Pattern>,
    pattern_impl: PatternImpl,
}

#[derive(Debug)]
enum PatternImpl {
    Solid(Color),
    Stripe(Color, Color),
    Gradient(Color, Color),
    Checker(Color, Color),
    Ring(Color, Color),
}

use PatternImpl::*;

impl Pattern {
    fn new(pattern_impl: PatternImpl) -> Self {
        Self {
            transform: Mat::identity(),
            pattern_impl,
        }
    }

    pub fn solid(c: Color) -> Self {
        Self::new(Solid(c))
    }

    pub fn stripe(a: Color, b: Color) -> Self {
        Self::new(Stripe(a, b))
    }

    pub fn gradient(a: Color, b: Color) -> Self {
        Self::new(Gradient(a, b - a))
    }

    pub fn checker(a: Color, b: Color) -> Self {
        Self::new(Checker(a, b))
    }

    pub fn ring(a: Color, b: Color) -> Self {
        Self::new(Ring(a, b))
    }

    /// Return an updated object with the given transform, where the transform is
    /// from pattern space to object space.
    pub fn with_transform(
        mut self,
        obj_to_pattern: Mat<4, spaces::Pattern, spaces::Object>,
    ) -> Self {
        self.transform = obj_to_pattern.inverse();
        self
    }

    /// Calculate the color at the given point in object space.
    pub fn color_at(&self, p: Point<spaces::Object>) -> Color {
        let p = self.transform * p;
        match self.pattern_impl {
            Solid(c) => c,
            Stripe(a, b) => {
                if p.x.rem_euclid(2.0) < 1.0 {
                    a
                } else {
                    b
                }
            }
            Gradient(a, b_minus_a) => a + (b_minus_a) * p.x.fract(),
            Checker(a, b) => {
                if (p.x.floor() + p.y.floor() + p.z.floor()).rem_euclid(2.0) < 1.0 {
                    a
                } else {
                    b
                }
            }
            Ring(a, b) => {
                let d = (p.x * p.x + p.z * p.z).sqrt();
                if d.rem_euclid(2.0) < 1.0 {
                    a
                } else {
                    b
                }
            }
        }
    }
}

impl From<Color> for Pattern {
    fn from(c: Color) -> Self {
        Self::solid(c)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;
    use approx::*;

    #[test]
    fn stripe_constant_y() {
        let p = Pattern::stripe(Color::white(), Color::black());
        assert_relative_eq!(p.color_at(Point::new(0, 0, 0)), Color::white());
        assert_relative_eq!(p.color_at(Point::new(0, 1, 0)), Color::white());
        assert_relative_eq!(p.color_at(Point::new(0, 2, 0)), Color::white());
    }

    #[test]
    fn stripe_constant_z() {
        let p = Pattern::stripe(Color::white(), Color::black());
        assert_relative_eq!(p.color_at(Point::new(0, 0, 0)), Color::white());
        assert_relative_eq!(p.color_at(Point::new(0, 0, 1)), Color::white());
        assert_relative_eq!(p.color_at(Point::new(0, 0, 2)), Color::white());
    }

    #[test]
    fn stripe_alternates_x() {
        let p = Pattern::stripe(Color::white(), Color::black());
        assert_relative_eq!(p.color_at(Point::new(0, 0, 0)), Color::white());
        assert_relative_eq!(p.color_at(Point::new(0.9, 0, 0)), Color::white());
        assert_relative_eq!(p.color_at(Point::new(1, 0, 0)), Color::black());
        assert_relative_eq!(p.color_at(Point::new(-0.1, 0, 0)), Color::black());
        assert_relative_eq!(p.color_at(Point::new(-1, 0, 0)), Color::black());
        assert_relative_eq!(p.color_at(Point::new(-1.1, 0, 0)), Color::white());
    }

    #[test]
    fn stripes_with_obj_transform() {
        let o = Object::new(Sphere)
            .with_transform(Mat::identity().scale(2, 2, 2))
            .with_material(Material {
                pattern: Pattern::stripe(Color::white(), Color::black()),
                ..Default::default()
            });
        assert_relative_eq!(o.color_at(Point::new(1.5, 0, 0)), Color::white());
    }

    #[test]
    fn stripes_with_pat_transform() {
        let o = Object::new(Sphere).with_material(Material {
            pattern: Pattern::stripe(Color::white(), Color::black())
                .with_transform(Mat::identity().scale(2, 2, 2)),
            ..Default::default()
        });
        assert_relative_eq!(o.color_at(Point::new(1.5, 0, 0)), Color::white());
    }

    #[test]
    fn stripes_with_both_transform() {
        let o = Object::new(Sphere)
            .with_transform(Mat::identity().scale(2, 2, 2))
            .with_material(Material {
                pattern: Pattern::stripe(Color::white(), Color::black())
                    .with_transform(Mat::identity().translate(0.5, 0, 0)),
                ..Default::default()
            });
        assert_relative_eq!(o.color_at(Point::new(1.5, 0, 0)), Color::white());
    }

    #[test]
    fn ring_both_x_and_z() {
        let p = Pattern::ring(Color::white(), Color::black());
        assert_relative_eq!(p.color_at(Point::new(0, 0, 0)), Color::white());
        assert_relative_eq!(p.color_at(Point::new(1, 0, 0)), Color::black());
        assert_relative_eq!(p.color_at(Point::new(0, 0, 1)), Color::black());
        assert_relative_eq!(p.color_at(Point::new(0.708, 0, 0.708)), Color::black());
    }

    #[test]
    fn checker_repeat_x() {
        let p = Pattern::checker(Color::white(), Color::black());
        assert_relative_eq!(p.color_at(Point::new(0, 0, 0)), Color::white());
        assert_relative_eq!(p.color_at(Point::new(0.99, 0, 0)), Color::white());
        assert_relative_eq!(p.color_at(Point::new(1.01, 0, 0)), Color::black());
    }

    #[test]
    fn checker_repeat_y() {
        let p = Pattern::checker(Color::white(), Color::black());
        assert_relative_eq!(p.color_at(Point::new(0, 0, 0)), Color::white());
        assert_relative_eq!(p.color_at(Point::new(0, 0.99, 0)), Color::white());
        assert_relative_eq!(p.color_at(Point::new(0, 1.01, 0)), Color::black());
    }

    #[test]
    fn checker_repeat_z() {
        let p = Pattern::checker(Color::white(), Color::black());
        assert_relative_eq!(p.color_at(Point::new(0, 0, 0)), Color::white());
        assert_relative_eq!(p.color_at(Point::new(0, 0, 0.99)), Color::white());
        assert_relative_eq!(p.color_at(Point::new(0, 0, 1.01)), Color::black());
    }

    #[test]
    fn gradient_linear_interp() {
        let p = Pattern::gradient(Color::white(), Color::black());
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
