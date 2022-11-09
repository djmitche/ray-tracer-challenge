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
    // TODO: support adding "entry" and "exit" intersections

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

    /// Get the first intersection with a non-negative `t`, along with the object _from_
    /// which that intersection occurs.
    pub fn hit(&mut self) -> (Option<ObjectIndex>, Option<&Intersection>) {
        self.sort();

        // TODO: do this without allocations?
        let mut containers: Vec<&Intersection> = Vec::new();
        let mut from_obj = None;

        for h in &self.hits {
            if h.t >= 0.0 {
                from_obj = containers.last().map(|i| i.object_index);
            }
            if let Some(container_idx) = containers
                .iter()
                .position(|i| i.object_index == h.object_index)
            {
                // this object was in containers, so we are exiting the object
                containers.remove(container_idx);
            } else {
                containers.push(h);
            }
            if h.t >= 0.0 {
                return (from_obj, containers.last().map(|iref| *iref));
            }
        }
        (from_obj, None)
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
        assert_relative_eq!(inters.hit().1.expect("a hit").t, 1.0);
    }

    #[test]
    fn hit_some_negative() {
        let mut inters = Intersections::default();
        inters.add(-1.0, ObjectIndex::test_value(1));
        inters.add(1.0, ObjectIndex::test_value(2));
        assert_relative_eq!(inters.hit().1.expect("a hit").t, 1.0);
    }

    #[test]
    fn hit_all_negative() {
        let mut inters = Intersections::default();
        inters.add(-1.0, ObjectIndex::test_value(1));
        inters.add(-2.0, ObjectIndex::test_value(1));
        assert!(inters.hit().1.is_none());
    }

    #[test]
    fn hit_lowest_nonneg() {
        let mut inters = Intersections::default();
        inters.add(5.0, ObjectIndex::test_value(1));
        inters.add(7.0, ObjectIndex::test_value(2));
        inters.add(-3.0, ObjectIndex::test_value(3));
        inters.add(2.0, ObjectIndex::test_value(4));
        assert_relative_eq!(inters.hit().1.expect("a hit").t, 2.0);
    }

    #[test]
    fn hit_with_from_obj() {
        //  |----------------a------------|
        //          |----b----|
        //              |----c----|
        // -3   -2   -1    0    1    2    3
        let mut w = World::default();
        let a = w.add_object(
            Object::new(Sphere)
                .with_material(Material::default().with_transparency(1.0, 1.5))
                .with_transform(Mat::identity().scale(3, 3, 3)),
        );
        let b = w.add_object(
            Object::new(Sphere)
                .with_material(Material::default().with_transparency(1.0, 2.0))
                .with_transform(Mat::identity().translate(0, 0, -0.25)),
        );
        let c = w.add_object(
            Object::new(Sphere)
                .with_material(Material::default().with_transparency(1.0, 2.5))
                .with_transform(Mat::identity().translate(0, 0, 0.25)),
        );

        fn idx(inter: Option<&Intersection>) -> Option<ObjectIndex> {
            inter.map(|i| i.object_index)
        }

        let mut inters = Intersections::default();
        let r: Ray<spaces::World> = Ray::new(Point::new(0, 0, -4), Vector::new(0, 0, 1));
        w.intersect(&r, &mut inters);
        let (from_obj, hit) = inters.hit();
        assert_eq!((from_obj, idx(hit)), (None, Some(a)));

        let mut inters = Intersections::default();
        let r: Ray<spaces::World> = Ray::new(Point::new(0, 0, -2), Vector::new(0, 0, 1));
        w.intersect(&r, &mut inters);
        let (from_obj, hit) = inters.hit();
        assert_eq!((from_obj, idx(hit)), (Some(a), Some(b)));

        let mut inters = Intersections::default();
        let r: Ray<spaces::World> = Ray::new(Point::new(0, 0, -1), Vector::new(0, 0, 1));
        w.intersect(&r, &mut inters);
        let (from_obj, hit) = inters.hit();
        assert_eq!((from_obj, idx(hit)), (Some(b), Some(c)));

        let mut inters = Intersections::default();
        let r: Ray<spaces::World> = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        w.intersect(&r, &mut inters);
        let (from_obj, hit) = inters.hit();
        assert_eq!((from_obj, idx(hit)), (Some(c), Some(c)));

        let mut inters = Intersections::default();
        let r: Ray<spaces::World> = Ray::new(Point::new(0, 0, 1), Vector::new(0, 0, 1));
        w.intersect(&r, &mut inters);
        let (from_obj, hit) = inters.hit();
        assert_eq!((from_obj, idx(hit)), (Some(c), Some(a)));

        let mut inters = Intersections::default();
        let r: Ray<spaces::World> = Ray::new(Point::new(0, 0, 2), Vector::new(0, 0, 1));
        w.intersect(&r, &mut inters);
        let (from_obj, hit) = inters.hit();
        assert_eq!((from_obj, idx(hit)), (Some(a), None));

        let mut inters = Intersections::default();
        let r: Ray<spaces::World> = Ray::new(Point::new(0, 0, 4), Vector::new(0, 0, 1));
        w.intersect(&r, &mut inters);
        let (from_obj, hit) = inters.hit();
        assert_eq!((from_obj, idx(hit)), (None, None));
    }
}
