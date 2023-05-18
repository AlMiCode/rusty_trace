use cgmath::{InnerSpace, Zero};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::vec_repo::{Id, VecRepo};

use super::{
    hittable::HitRecord, random_f64, random_vec_in_sphere, texture::Texture, Colour, Ray, Vector3,
};

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

#[enum_dispatch(MaterialTrait)]
#[derive(Clone, Serialize, Deserialize)]
pub enum Material {
    Lambertian,
    Metal,
    Dielectric,
    DiffuseLight,
    Isotropic,
}

impl Default for Material {
    fn default() -> Self {
        Material::from(Lambertian {
            albedo: Id::default(),
        })
    }
}

#[enum_dispatch]
pub trait MaterialTrait {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &HitRecord,
        textures: &VecRepo<Texture>,
    ) -> Option<ScatterRecord>;
    fn emit(&self, _u: f64, _v: f64, _textures: &VecRepo<Texture>) -> Colour {
        Colour::zero()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Lambertian {
    pub albedo: Id<Texture>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Metal {
    pub albedo: Id<Texture>,
    pub fuzz: f64,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Dielectric {
    pub refractive_index: f64,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct DiffuseLight {
    pub emit: Id<Texture>,
    pub amplify: f32,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Isotropic {
    pub albedo: Id<Texture>,
}

impl MaterialTrait for Lambertian {
    fn scatter(
        &self,
        _ray: &Ray,
        hit: &HitRecord,
        textures: &VecRepo<Texture>,
    ) -> Option<ScatterRecord> {
        let scatter_dir = hit.normal + random_vec_in_sphere();
        let scatter_dir = if scatter_dir.magnitude2() < 0.000001 {
            hit.normal
        } else {
            scatter_dir.normalize()
        };
        Some(ScatterRecord {
            ray: Ray::new(hit.point, scatter_dir),
            attenuation: textures.get(self.albedo).colour_at(hit.uv.0, hit.uv.1),
        })
    }
}

impl MaterialTrait for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &HitRecord,
        textures: &VecRepo<Texture>,
    ) -> Option<ScatterRecord> {
        let dir = reflect(&ray.direction, &hit.normal) + random_vec_in_sphere() * self.fuzz;
        if dir.dot(hit.normal) > 0.0 {
            Some(ScatterRecord {
                ray: Ray::new(hit.point, dir),
                attenuation: textures.get(self.albedo).colour_at(hit.uv.0, hit.uv.1),
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

impl MaterialTrait for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &HitRecord,
        _textures: &VecRepo<Texture>,
    ) -> Option<ScatterRecord> {
        let refraction_ratio = if hit.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let cos_theta = hit.normal.dot(-ray.direction).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = sin_theta * cos_theta > 1.0;
        let scattered = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random_f64(0.0, 1.0)
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

impl MaterialTrait for DiffuseLight {
    fn scatter(
        &self,
        _ray: &Ray,
        _hit: &HitRecord,
        _textures: &VecRepo<Texture>,
    ) -> Option<ScatterRecord> {
        None
    }
    fn emit(&self, u: f64, v: f64, textures: &VecRepo<Texture>) -> Colour {
        textures.get(self.emit).colour_at(u, v) * self.amplify
    }
}

impl MaterialTrait for Isotropic {
    fn scatter(
        &self,
        _ray: &Ray,
        hit: &HitRecord,
        textures: &VecRepo<Texture>,
    ) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            ray: Ray::new(hit.point, random_vec_in_sphere()),
            attenuation: textures.get(self.albedo).colour_at(hit.uv.0, hit.uv.1),
        })
    }
}
