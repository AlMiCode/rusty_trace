use crate::{Ray, Colour, random_vec_in_hemisphere};
use crate::hittable::HitRecord;

pub struct ScatterRecord {
    pub ray: Ray,
    pub attenuation: Colour
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord>;
}

pub struct Lambertian {
    pub albedo: Colour
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let scatter_dir = random_vec_in_hemisphere(hit.normal.clone());
        Some(ScatterRecord { 
            ray: Ray::new(hit.point, scatter_dir), 
            attenuation: self.albedo
        })
    }
}