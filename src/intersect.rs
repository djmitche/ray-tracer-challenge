use crate::Object;

/// An intersection represents the point at which a ray intersects
/// an object.
#[derive(Debug, Copy, Clone)]
pub struct Intersection<'o> {
    /// The position along the ray at which the intersection occurs, as a
    /// multiple of the length of the ray.
    pub t: f64,

    /// The intersected object
    pub obj: &'o Object,
}

impl<'o> Intersection<'o> {
    pub fn new(t: impl Into<f64>, obj: &'o Object) -> Self {
        Self { t: t.into(), obj }
    }
}

/// Itersections maintains a mutable set of Intersection instances
#[derive(Debug)]
pub struct Intersections<'o> {
    /// the object currently being intersected
    cur_object: Option<&'o Object>,

    /// Recorded hits
    hits: Vec<Intersection<'o>>,

    /// Tracking for whether this set of hits is sorted
    sorted: bool,
}

impl Default for Intersections<'_> {
    fn default() -> Self {
        Self {
            cur_object: None,
            hits: Vec::new(),
            sorted: true,
        }
    }
}

impl<'o> Intersections<'o> {
    /// Set the object that will be associated with subsequent `add` calls
    pub fn set_object(&mut self, object: &'o Object) {
        self.cur_object = Some(object);
    }

    /// Reset the current object
    pub fn unset_object(&mut self) {
        self.cur_object = None;
    }

    /// Add a new intersection to this set, using the current object
    pub fn add(&mut self, t: f64) {
        self.hits.push(Intersection::new(
            t,
            self.cur_object.expect("expected a current object"),
        ));
        self.sorted = false;
    }

    pub fn len(&self) -> usize {
        self.hits.len()
    }

    fn sort(&mut self) {
        if !self.sorted {
            self.hits
                .sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal));
            self.sorted = true;
        }
    }

    /// Get the nearest intersection
    pub fn hit(&mut self) -> Option<&Intersection<'o>> {
        self.sort();
        self.hits.iter().skip_while(|i| i.t < 0.0).nth(0)
    }

    /// Get an iterator over all hits, in order by `t`
    pub fn iter(&mut self) -> std::slice::Iter<Intersection<'o>> {
        self.sort();
        self.hits.iter()
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use approx::*;

    #[test]
    fn intersection_contains_object() {
        let s = Object::new(Sphere);
        let i = Intersection::new(1, &s);
        assert_relative_eq!(i.t, 1.0);
        // can't test equality of dyn refs
        // assert_eq!(i.obj, &s);
    }

    #[test]
    fn hit_all_positive_t() {
        let s = Object::new(Sphere);
        let mut inters = Intersections::default();
        inters.set_object(&s);
        inters.add(1.0);
        inters.add(2.0);
        assert_relative_eq!(inters.hit().expect("a hit").t, 1.0);
    }

    #[test]
    fn hit_some_negative() {
        let s = Object::new(Sphere);
        let mut inters = Intersections::default();
        inters.set_object(&s);
        inters.add(-1.0);
        inters.add(1.0);
        assert_relative_eq!(inters.hit().expect("a hit").t, 1.0);
    }

    #[test]
    fn hit_all_negative() {
        let s = Object::new(Sphere);
        let mut inters = Intersections::default();
        inters.set_object(&s);
        inters.add(-1.0);
        inters.add(-2.0);
        assert!(inters.hit().is_none());
    }

    #[test]
    fn hit_lowest_nonneg() {
        let s = Object::new(Sphere);
        let mut inters = Intersections::default();
        inters.set_object(&s);
        inters.add(5.0);
        inters.add(7.0);
        inters.add(-3.0);
        inters.add(2.0);
        assert_relative_eq!(inters.hit().expect("a hit").t, 2.0);
    }
}
