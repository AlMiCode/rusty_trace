use cgmath::InnerSpace;

use crate::hittable::HitRecord;
use crate::{random_vec_in_sphere, Colour, Ray};

pub struct ScatterRecord {
    pub ray: Ray,
    pub attenuation: Colour,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord>;
}

pub struct Lambertian {
    pub albedo: Colour,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let scatter_dir = hit.normal + random_vec_in_sphere();
        let scatter_dir = if scatter_dir.magnitude2() < 0.000001 {
            hit.normal
        } else {
            scatter_dir.normalize()
        };
        Some(ScatterRecord {
            ray: Ray::new(hit.point, scatter_dir),
            attenuation: self.albedo,
        })
    }
}
