use crate::{spaces, Color, PatternFn, Point};

#[derive(Debug)]
pub struct Solid(Color);

impl Solid {
    pub fn new(c: Color) -> Self {
        Self(c)
    }
}

impl PatternFn for Solid {
    fn color_at(&self, _: Point<spaces::Pattern>) -> Color {
        self.0
    }
}
