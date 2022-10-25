use crate::ObjectIndex;

/// An intersection represents the point at which a ray intersects
/// an object.
#[derive(Debug, Copy, Clone)]
pub struct Intersection {
    /// The position along the ray at which the intersection occurs, as a
    /// multiple of the length of the ray.
    pub t: f64,

    /// The intersected object
    pub object_index: ObjectIndex,
}

/// Itersections maintains a mutable set of Intersection instances
#[derive(Debug)]
pub struct Intersections {
    /// Recorded hits
    hits: Vec<Intersection>,

    /// Tracking for whether this set of hits is sorted
    sorted: bool,
}

impl Default for Intersections {
    fn default() -> Self {
        Self {
            hits: Vec::new(),
            sorted: true,
        }
    }
}

impl Intersections {
    /// Add a new intersection to this set, using the current object
    pub fn add(&mut self, t: f64, object_index: ObjectIndex) {
        self.hits.push(Intersection { t, object_index });
        self.sorted = false;
    }

    pub fn len(&self) -> usize {
        self.hits.len()
    }

    pub fn clear(&mut self) {
        self.hits.clear();
    }

    fn sort(&mut self) {
        if !self.sorted {
            self.hits
                .sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal));
            self.sorted = true;
        }
    }

    /// Get the nearest intersection
    pub fn hit(&mut self) -> Option<&Intersection> {
        self.sort();
        self.hits.iter().skip_while(|i| i.t < 0.0).nth(0)
    }

    /// Get an iterator over all hits, in order by `t`
    pub fn iter(&mut self) -> std::slice::Iter<Intersection> {
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
        let i = Intersection {
            t: 1.0,
            object_index: ObjectIndex::test_value(0),
        };
        assert_relative_eq!(i.t, 1.0);
        assert_eq!(i.object_index, ObjectIndex::test_value(0));
    }

    #[test]
    fn hit_all_positive_t() {
        let mut inters = Intersections::default();
        inters.add(1.0, ObjectIndex::test_value(1));
        inters.add(2.0, ObjectIndex::test_value(1));
        assert_relative_eq!(inters.hit().expect("a hit").t, 1.0);
    }

    #[test]
    fn hit_some_negative() {
        let mut inters = Intersections::default();
        inters.add(-1.0, ObjectIndex::test_value(1));
        inters.add(1.0, ObjectIndex::test_value(1));
        assert_relative_eq!(inters.hit().expect("a hit").t, 1.0);
    }

    #[test]
    fn hit_all_negative() {
        let mut inters = Intersections::default();
        inters.add(-1.0, ObjectIndex::test_value(1));
        inters.add(-2.0, ObjectIndex::test_value(1));
        assert!(inters.hit().is_none());
    }

    #[test]
    fn hit_lowest_nonneg() {
        let mut inters = Intersections::default();
        inters.add(5.0, ObjectIndex::test_value(1));
        inters.add(7.0, ObjectIndex::test_value(1));
        inters.add(-3.0, ObjectIndex::test_value(1));
        inters.add(2.0, ObjectIndex::test_value(1));
        assert_relative_eq!(inters.hit().expect("a hit").t, 2.0);
    }
}
