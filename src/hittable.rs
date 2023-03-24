use crate::material::Material;
use crate::repo::Id;
use crate::{Point3, Ray, Vector3};
use cgmath::InnerSpace;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vector3,
    pub distance: f64,
    pub uv: (f64, f64),
    pub front_face: bool,
    pub material_id: Id<dyn Material>,
}

pub trait Hittable: Sync + Send + HittableClone {
    fn hit_bounded(&self, ray: &Ray, min_dist: f64, max_dist: f64) -> Option<HitRecord>;
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        self.hit_bounded(ray, f64::EPSILON, f64::INFINITY)
    }
    fn get_position(&self) -> Point3;
    fn set_position(&mut self, c: Point3);
}
pub type HittableVec = Vec<Box<dyn Hittable>>;

pub trait HittableClone {
    fn clone_box(&self) -> Box<dyn Hittable>;
}

impl<T> HittableClone for T
where
    T: 'static + Hittable + Clone,
{
    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Box<dyn Hittable> {
        self.clone_box()
    }
}

impl Hittable for HittableVec {
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

    fn get_position(&self) -> Point3 { unimplemented!() }
    fn set_position(&mut self, _c: Point3) { unimplemented!() }
}

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material_id: Id<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material_id: Id<dyn Material>) -> Self {
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
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        let result = HitRecord {
            point,
            normal,
            distance: root,
            front_face,
            uv: Sphere::get_uv(&outward_normal),
            material_id: self.material_id,
        };

        Some(result)
    }

    fn get_position(&self) -> Point3 { self.center }
    fn set_position(&mut self, c: Point3) {
        self.center = c;
    }
}
