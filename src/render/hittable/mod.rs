use cgmath::InnerSpace;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::vec_repo::Id;

use super::{material::Material, Point3, Ray, Vector3};

pub(crate) mod modifiers;
pub(crate) mod rect;
pub(crate) mod sphere;

use modifiers::*;
use rect::Rect;
use sphere::Sphere;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vector3,
    pub distance: f64,
    pub uv: (f64, f64),
    pub front_face: bool,
    pub material_id: Id<Material>,
}

impl HitRecord {
    fn new(
        ray: &Ray,
        distance: f64,
        outward_normal: Vector3,
        uv: (f64, f64),
        material_id: Id<Material>,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            point: ray.at(distance),
            normal,
            distance,
            uv,
            front_face,
            material_id,
        }
    }
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

#[enum_dispatch(HittableTrait)]
#[derive(Clone, Serialize, Deserialize)]
pub enum Hittable {
    Sphere,
    Rect,
    Translate,
    RotateY,
}

#[enum_dispatch]
pub trait HittableTrait {
    fn name(&self) -> &'static str;
    fn hit_bounded(&self, ray: &Ray, min_dist: f64, max_dist: f64) -> Option<HitRecord>;
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        self.hit_bounded(ray, f64::EPSILON, f64::INFINITY)
    }
    fn get_position(&self) -> Point3;
    fn set_position(&mut self, c: Point3);
}

pub type HittableVec = Vec<Hittable>;
impl HittableTrait for HittableVec {
    fn hit_bounded(&self, ray: &Ray, min_dist: f64, max_dist: f64) -> Option<HitRecord> {
        let mut result = None;
        let mut closest_dist = f64::INFINITY;
        for shape in self {
            let hit = shape.hit_bounded(ray, min_dist, max_dist);
            if let Some(hit) = hit {
                if hit.distance < closest_dist {
                    closest_dist = hit.distance;
                    result = Some(hit);
                }
            }
        }
        result
    }

    fn get_position(&self) -> Point3 {
        unimplemented!()
    }
    fn set_position(&mut self, _c: Point3) {
        unimplemented!()
    }

    fn name(&self) -> &'static str {
        "Group"
    }
}
