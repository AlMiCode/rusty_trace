use cgmath::InnerSpace;
use crate::{Point3, Ray};

pub trait HitObject {
    fn hit(&self, ray: &Ray) -> f64;
}

pub enum ShapeEnum {
    Sphere(Sphere),
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl HitObject for Sphere {
    fn hit(&self, ray: &Ray) -> f64 {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            -1.0
        } else {
            ( -b - discriminant.sqrt() ) / 2.0*a
        }
    }
}
