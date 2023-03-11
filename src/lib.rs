use camera::Camera;
use cgmath::{vec3, InnerSpace, point3};
use image::{Rgb, RgbImage};
use hittable::{Hittable, HittableVec};

pub mod gui;
pub mod hittable;
pub mod camera;

pub type Point3 = cgmath::Point3<f64>;
pub type Vector3 = cgmath::Vector3<f64>;
pub type Colour = cgmath::Vector3<f64>;

pub fn render(image: &mut RgbImage, camera: &Camera, scene: &HittableVec) {
    let (width, height) = image.dimensions();
    for y in 0..height {
        for x in 0..width {
            let u = x as f64 / (width - 1) as f64;
            let v = y as f64 / (height - 1) as f64;
            let r = camera.get_ray(u, v);
            let pixel: Rgb<u8> = vec_to_rgb(ray_colour(r, scene));
            image.put_pixel(x, height - y - 1, pixel);
        }
    }
}

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

pub fn ray_colour(ray: Ray, hit_vec: &HittableVec) -> Colour {
    let t = hit_vec.hit(&ray);
    if t > 0.0 {
        let normal_vec = (ray.at(t) - point3(0.0,0.0,-1.0)).normalize();
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
