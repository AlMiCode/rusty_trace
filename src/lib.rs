use cgmath::{InnerSpace, point3, vec3};
use image::Rgb;

pub mod gui;

type Point3 = cgmath::Point3<f64>;
type Vector3 = cgmath::Vector3<f64>;
type Colour = cgmath::Vector3<f64>;

pub struct Ray {
    origin: Point3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Ray { origin, direction }
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

pub fn ray_colour(ray: Ray) -> Colour {
    if hit_sphere(point3(0.0 ,0.0 ,-1.0), 0.5, &ray) {
        vec3(1.0, 0.0, 0.0)
    } else {
        let unit_dir = ray.direction.normalize();
        let t = 0.5 * (unit_dir.y + 1.0);
        (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0)
    }
}

pub fn vec_to_rgb(vec: Colour) -> Rgb<u8> {
    Rgb(vec.map(|n| (n.clamp(0.0, 1.0) * 255.0) as u8).into())
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius*radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}