use crate::{spaces, Color, Mat, Point};

mod solid;
mod stripe;

pub use solid::*;
pub use stripe::*;

pub trait PatternFn: std::fmt::Debug + Sync + Send {
    fn color_at(&self, point: Point<spaces::Pattern>) -> Color;
}

/// Pattern defines a pattern of colors in object space
#[derive(Debug)]
pub struct Pattern {
    transform: Mat<4, spaces::Object, spaces::Pattern>,
    pattern_fn: Box<dyn PatternFn>,
}

impl Pattern {
    pub fn new<PF: PatternFn + 'static>(pattern_fn: PF) -> Pattern {
        Pattern {
            transform: Mat::identity(),
            pattern_fn: Box::new(pattern_fn),
        }
    }

    pub fn solid(c: Color) -> Pattern {
        Self::new(Solid::new(c))
    }

    pub fn stripe(a: Color, b: Color) -> Pattern {
        Self::new(Stripe::new(a, b))
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
        // handle kinds that don't require a point
        let p = self.transform * p;
        self.pattern_fn.color_at(p)
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
}
