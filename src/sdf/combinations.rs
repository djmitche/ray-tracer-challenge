use crate::sdf::Sdf;
use crate::{spaces, Point};

/// This macro creates a collection of Unions containing all of the given SDFs.
#[macro_export]
macro_rules! union {
    ($e:expr) => {
        $e
    };

    ($e:expr, $($rest:expr),+) => {
        $crate::sdf::Union::new($e, union!( $($rest),+ ))
    };

    ($e:expr, $($rest:expr),+,) => {
        $crate::sdf::Union::new($e, union!( $($rest),+ ))
    };
}

/// Union combines child SDFs with a union operation.
pub struct Union<SDF1: Sdf, SDF2: Sdf>(SDF1, SDF2);

impl<SDF1: Sdf, SDF2: Sdf> Union<SDF1, SDF2> {
    pub fn new(a: SDF1, b: SDF2) -> Self {
        Self(a, b)
    }
}

impl<SDF1: Sdf, SDF2: Sdf> Sdf for Union<SDF1, SDF2> {
    fn distance(&self, point: &Point<spaces::World>) -> f64 {
        self.0.distance(point).min(self.1.distance(point))
    }
}

/// This macro creates a collection of Intersections containing all of the given SDFs.
#[macro_export]
macro_rules! intersection {
    ($e:expr) => {
        $e
    };

    ($e:expr, $($rest:expr),+) => {
        $crate::sdf::Intersection::new($e, intersection!( $($rest),+ ))
    };

    ($e:expr, $($rest:expr),+,) => {
        $crate::sdf::Intersection::new($e, intersection!( $($rest),+ ))
    };
}

/// Intersection combines child SDFs with a intersection operation.
pub struct Intersection<SDF1: Sdf, SDF2: Sdf>(SDF1, SDF2);

impl<SDF1: Sdf, SDF2: Sdf> Intersection<SDF1, SDF2> {
    pub fn new(a: SDF1, b: SDF2) -> Self {
        Self(a, b)
    }
}

impl<SDF1: Sdf, SDF2: Sdf> Sdf for Intersection<SDF1, SDF2> {
    fn distance(&self, point: &Point<spaces::World>) -> f64 {
        self.0.distance(point).max(self.1.distance(point))
    }
}
