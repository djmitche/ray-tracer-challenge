use crate::{spaces, Color, PatternFn, Point};

#[derive(Debug)]
pub struct Stripe(Color, Color);

impl Stripe {
    pub fn new(a: Color, b: Color) -> Self {
        Self(a, b)
    }
}

impl PatternFn for Stripe {
    fn color_at(&self, p: Point<spaces::Pattern>) -> Color {
        if p.x.rem_euclid(2.0) < 1.0 {
            self.0
        } else {
            self.1
        }
    }
}
