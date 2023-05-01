use cgmath::EuclideanSpace;

use crate::render::{Point3, Ray, Vector3};

use super::{HitRecord, Hittable};

#[derive(Clone)]
struct Translate {
    offset: Vector3,
    object: Box<dyn Hittable>,
}

impl Translate {
    fn new(object: Box<dyn Hittable>, offset: Vector3) -> Self {
        Self { object, offset }
    }
}

impl Hittable for Translate {
    fn hit_bounded(&self, ray: &Ray, min_dist: f64, max_dist: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction);
        self.object
            .hit_bounded(&moved_ray, min_dist, max_dist)
            .and_then(|mut hit| {
                hit.point += self.offset;
                hit.set_face_normal(ray, hit.normal);
                Some(hit)
            })
    }

    fn set_position(&mut self, c: Point3) {
        self.offset = c.to_vec();
    }

    fn get_position(&self) -> Point3 {
        Point3::from_vec(self.offset)
    }

    fn name(&self) -> &'static str {
        "Translate"
    }
}
