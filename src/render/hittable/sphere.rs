use cgmath::InnerSpace;

use crate::{render::{material::Material, Point3, Ray, Vector3}, vec_repo::Id};

use super::{HitRecord, Hittable};

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    material_id: Id<Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material_id: Id<Material>) -> Self {
        Sphere {
            center,
            radius,
            material_id,
        }
    }

    pub fn get_uv(normal: &Vector3) -> (f64, f64) {
        let pi = std::f64::consts::PI;
        let phi = (-normal.z).atan2(normal.x) + pi;
        let u = phi / (2.0 * pi);
        let v = normal.y.asin() / pi + 0.5;
        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit_bounded(&self, ray: &Ray, min_dist: f64, max_dist: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let half_b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;
        if root < min_dist || root > max_dist {
            root = (-half_b + sqrt_d) / a;
            if root < min_dist || root > max_dist {
                return None;
            }
        }
        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;
        Some(HitRecord::new(
            ray,
            root,
            outward_normal,
            Sphere::get_uv(&outward_normal),
            self.material_id,
        ))
    }

    fn get_position(&self) -> Point3 {
        self.center
    }
    fn set_position(&mut self, c: Point3) {
        self.center = c;
    }

    fn name(&self) -> &'static str {
        "Sphere"
    }
}
