use cgmath::InnerSpace;
use crate::{Point3, Ray};

pub trait Hittable{
    fn hit(&self, ray: &Ray) -> f64;
}

pub type HittableVec = Vec<Box<dyn Hittable>>;

impl Hittable for HittableVec {
    fn hit(&self, ray: &Ray) -> f64 {
       let mut t = -1.0;
        for shape in self {
            let t1 = shape.hit(&ray);
            if t1 == -1.0 { continue }
            if t == -1.0 { t = t1 } else if t1 < t { t = t1 }
        }
        t
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
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
