use cgmath::{vec3, InnerSpace};
use image::Rgb;
use shapes::Hittable;

pub mod gui;
pub mod scene;
pub mod shapes;

pub type Point3 = cgmath::Point3<f64>;
pub type Vector3 = cgmath::Vector3<f64>;
pub type Colour = cgmath::Vector3<f64>;

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

pub fn ray_colour(ray: Ray, hittable: &Vec<Box<dyn Hittable>>) -> Colour {
    let mut t = -1.0;
    for shape in hittable {
        let t1 = shape.hit(&ray);
        if t1 == -1.0 { continue }
        if t == -1.0 { t = t1 } else if t1 < t { t = t1 }
    }
    if t > 0.0 {
        let normal_vec = ray.at(t) - Vector3::new(0.0,0.0,-1.0);
        let normal_vec = Vector3::new(normal_vec.x, normal_vec.y, normal_vec.z).normalize();
        0.5 * (normal_vec + Vector3::new(1.0, 1.0, 1.0))
    } else {
        let unit_dir = ray.direction.normalize();
        let t = 0.5 * (unit_dir.y + 1.0);
        (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0)
    }
}

pub fn vec_to_rgb(vec: Colour) -> Rgb<u8> {
    Rgb(vec.map(|n| (n.clamp(0.0, 1.0) * 255.0) as u8).into())
}
