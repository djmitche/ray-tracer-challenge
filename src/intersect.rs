use crate::Object;

/// An intersection represents the point at which a ray intersects
/// an object.
#[derive(Debug, Copy, Clone)]
pub struct Intersection<'o> {
    /// The position along the ray at which the intersection occurs
    pub t: f64,

    /// The intersected object
    pub obj: &'o dyn Object,
}

impl<'o> Intersection<'o> {
    pub fn new(t: impl Into<f64>, obj: &'o dyn Object) -> Self {
        Self { t: t.into(), obj }
    }
}

/// Itersections maintains a mutable set of Intersection instances
#[derive(Debug)]
pub struct Intersections<'o>(Vec<Intersection<'o>>);

impl Default for Intersections<'_> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<'o> Intersections<'o> {
    /// Add a new intersection to this set.
    pub fn add(&mut self, inter: Intersection<'o>) {
        self.0.push(inter)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Get the nearest intersection
    pub fn hit(&self) -> Option<&Intersection<'o>> {
        return self.0.iter().fold(None, |acc, inter| {
            if inter.t > 0.0 {
                if let Some(Intersection { t, .. }) = acc {
                    if *t < inter.t {
                        acc
                    } else {
                        Some(inter)
                    }
                } else {
                    Some(inter)
                }
            } else {
                acc
            }
        });
    }

    /// Get an iterator over all hits, in the order they were added.
    pub fn iter(&self) -> std::slice::Iter<Intersection<'o>> {
        self.0.iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Sphere;
    use approx::*;

    #[test]
    fn intersection_contains_object() {
        let s = Sphere::default();
        let i = Intersection::new(1, &s);
        assert_relative_eq!(i.t, 1.0);
        // can't test equality of dyn refs
        // assert_eq!(i.obj, &s);
    }

    #[test]
    fn hit_all_positive_t() {
        let s = Sphere::default();
        let mut inters = Intersections::default();
        inters.add(Intersection::new(1, &s));
        inters.add(Intersection::new(2, &s));
        assert_relative_eq!(inters.hit().expect("a hit").t, 1.0);
    }

    #[test]
    fn hit_some_negative() {
        let s = Sphere::default();
        let mut inters = Intersections::default();
        inters.add(Intersection::new(-1, &s));
        inters.add(Intersection::new(1, &s));
        assert_relative_eq!(inters.hit().expect("a hit").t, 1.0);
    }

    #[test]
    fn hit_all_negative() {
        let s = Sphere::default();
        let mut inters = Intersections::default();
        inters.add(Intersection::new(-1, &s));
        inters.add(Intersection::new(-2, &s));
        assert!(inters.hit().is_none());
    }

    #[test]
    fn hit_lowest_nonneg() {
        let s = Sphere::default();
        let mut inters = Intersections::default();
        inters.add(Intersection::new(5, &s));
        inters.add(Intersection::new(7, &s));
        inters.add(Intersection::new(-3, &s));
        inters.add(Intersection::new(2, &s));
        assert_relative_eq!(inters.hit().expect("a hit").t, 2.0);
    }
}
