use crate::{Color, Intersections, Light, Object, Ray, Tup};

#[derive(Debug)]
pub struct World {
    pub light: Light,
    pub objects: Vec<Box<dyn Object>>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            light: Light::new_point(Tup::point(-10, 10, -10), Color::white()),
            objects: vec![],
        }
    }
}

impl World {
    pub fn add(&mut self, obj: Box<dyn Object>) {
        self.objects.push(obj);
    }

    pub fn intersect<'o>(&'o self, ray: &Ray, inters: &mut Intersections<'o>) {
        for o in &self.objects {
            o.intersect(ray, inters);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;
    use approx::*;

    fn default_world() -> World {
        let mut w = World::default();
        w.add(Box::new(Sphere::default().with_material(Material {
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        })));
        w.add(Box::new(
            Sphere::default()
                .with_transform(Mat::identity().scale(0.5, 0.5, 0.5))
                .with_material(Material::default()),
        ));
        w
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = default_world();
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let mut inters = Intersections::default();
        w.intersect(&r, &mut inters);
        let mut it = inters.iter();
        assert_relative_eq!(it.next().expect("intersection").t, 4.0);
        assert_relative_eq!(it.next().expect("intersection").t, 4.5);
        assert_relative_eq!(it.next().expect("intersection").t, 5.5);
        assert_relative_eq!(it.next().expect("intersection").t, 6.0);
        assert!(it.next().is_none());
    }
}
