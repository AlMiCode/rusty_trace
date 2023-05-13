use cgmath::EuclideanSpace;

use crate::render::{Point3, Ray, Vector3};

use super::{HitRecord, Hittable};

#[derive(Clone)]
pub struct Translate {
    offset: Vector3,
    object: Box<dyn Hittable>,
}

impl Translate {
    pub fn new(object: Box<dyn Hittable>, offset: Vector3) -> Self {
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

#[derive(Clone)]
pub struct RotateY {
    sin_y: f64,
    cos_y: f64,
    object: Box<dyn Hittable>,
}

impl RotateY {
    pub fn new(object: Box<dyn Hittable>, degrees: f64) -> Self {
        let angle = degrees.to_radians();
        Self {
            sin_y: angle.sin(),
            cos_y: angle.cos(),
            object,
        }
    }
}

impl Hittable for RotateY {
    fn hit_bounded(&self, ray: &Ray, min_dist: f64, max_dist: f64) -> Option<HitRecord> {
        let Ray {
            mut origin,
            mut direction,
        } = ray;

        origin[0] = self.cos_y * ray.origin[0] - self.sin_y * ray.origin[2];
        origin[2] = self.sin_y * ray.origin[0] + self.cos_y * ray.origin[2];

        direction[0] = self.cos_y * ray.direction[0] - self.sin_y * ray.direction[2];
        direction[2] = self.sin_y * ray.direction[0] + self.cos_y * ray.direction[2];

        let rotated_ray = Ray::new(origin, direction);
        let hit = self.object.hit_bounded(&rotated_ray, min_dist, max_dist);
        if hit.is_none() {
            return None;
        };
        let mut hit = hit.unwrap();

        let mut point = hit.point;
        let mut normal = hit.normal;
        point[0] = self.cos_y * hit.point[0] + self.sin_y * hit.point[2];
        point[2] = -self.sin_y * hit.point[0] + self.cos_y * hit.point[2];

        normal[0] = self.cos_y * hit.normal[0] + self.sin_y * hit.normal[2];
        normal[2] = -self.sin_y * hit.normal[0] + self.cos_y * hit.normal[2];

        hit.point = point;
        hit.set_face_normal(&rotated_ray, normal);
        Some(hit)
    }

    fn get_position(&self) -> Point3 {
        unimplemented!()
    }

    fn set_position(&mut self, _c: Point3) {
        unimplemented!()
    }

    fn name(&self) -> &'static str {
        "RotateY"
    }
}
