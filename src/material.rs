use std::sync::Arc;

use cgmath::{InnerSpace, Zero};

use crate::hittable::HitRecord;
use crate::resource_manager::ResourceManager;
use crate::texture::Texture;
use crate::{random_f64, random_vec_in_sphere, Colour, Ray, Vector3};

fn reflect(vec: &Vector3, normal: &Vector3) -> Vector3 {
    vec - normal * 2.0 * vec.dot(normal.clone())
}

fn refract(vec: &Vector3, normal: &Vector3, refractive_ratio: f64) -> Vector3 {
    let cos_theta = normal.dot(-vec.clone()).min(1.0);
    let r_perp = (vec + normal * cos_theta) * refractive_ratio;
    let r_para = normal * -(1.0 - r_perp.magnitude2()).abs().sqrt();
    r_para + r_perp
}

pub struct ScatterRecord {
    pub ray: Ray,
    pub attenuation: Colour,
}

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord>;
    fn emit(&self, _u: f64, _v: f64) -> Colour {
        Colour::zero()
    }
}

pub type MaterialManager = ResourceManager<dyn Material>;

pub struct Lambertian {
    pub albedo: Arc<Texture>,
}

pub struct Metal {
    pub albedo: Arc<Texture>,
    pub fuzz: f64,
}
pub struct Dielectric {
    pub refractive_index: f64,
}
pub struct DiffuseLight {
    pub emit: Arc<Texture>,
}
pub struct Isotropic {
    pub albedo: Arc<Texture>,
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
            attenuation: self.albedo.colour_at(hit.uv.0, hit.uv.1),
        })
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let dir = reflect(&ray.direction, &hit.normal) + random_vec_in_sphere() * self.fuzz;
        if dir.dot(hit.normal) > 0.0 {
            Some(ScatterRecord {
                ray: Ray::new(hit.point, dir),
                attenuation: self.albedo.colour_at(hit.uv.0, hit.uv.1),
            })
        } else {
            None
        }
    }
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio = if hit.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let cos_theta = hit.normal.dot(-ray.direction).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = sin_theta * cos_theta > 1.0;
        let scattered = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random_f64()
        {
            reflect(&ray.direction, &hit.normal)
        } else {
            refract(&ray.direction, &hit.normal, refraction_ratio)
        };
        Some(ScatterRecord {
            ray: Ray::new(hit.point, scattered.normalize()),
            attenuation: Colour::from((1.0, 1.0, 1.0)),
        })
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord) -> Option<ScatterRecord> {
        None
    }
    fn emit(&self, u: f64, v: f64) -> Colour {
        self.emit.colour_at(u, v)
    }
}

impl Material for Isotropic {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            ray: Ray::new(hit.point, random_vec_in_sphere()),
            attenuation: self.albedo.colour_at(hit.uv.0, hit.uv.1),
        })
    }
}
