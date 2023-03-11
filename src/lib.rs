use camera::Camera;
use cgmath::{ElementWise, InnerSpace, Zero};
use hittable::{Hittable, HittableVec};
use image::{Rgb, RgbImage};
use rand::Rng;

use std::io::Write;

pub mod camera;
pub mod gui;
pub mod hittable;
pub mod material;

pub type Point3 = cgmath::Point3<f64>;
pub type Vector3 = cgmath::Vector3<f64>;
pub type Colour = cgmath::Vector3<f64>;

pub fn render(image: &mut RgbImage, camera: &Camera, scene: &HittableVec, background: Colour, sample_count: u32) {
    let (width, height) = image.dimensions();
    for y in 0..height {
        for x in 0..width {
            let mut colour = Colour::zero();
            for _s in 0..sample_count {
                let u = x as f64 / (width - 1) as f64;
                let v = y as f64 / (height - 1) as f64;
                let r = camera.get_ray(u, v);
                
                colour += cast_ray(r, scene, background, 30)
            }
            let pixel: Rgb<u8> = vec_to_rgb(gamma_correction(colour / sample_count as f64));
            image.put_pixel(x, height - y - 1, pixel);
        }
        print!("\r{}/{} done", y+1, height); std::io::stdout().flush();
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

pub fn cast_ray(ray: Ray, hittable: &dyn Hittable, background: Colour, depth: u32) -> Colour {
    if depth == 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }
    if let Some(hit) = hittable.hit(&ray) {
        match hit.material.scatter(&ray, &hit) {
            None => Colour::new(0.0, 0.0, 0.0),
            Some(scattered) => scattered.attenuation.mul_element_wise(cast_ray(
                scattered.ray,
                hittable,
                background,
                depth - 1,
            )),
        }
    } else {
        background
    }
}

fn gamma_correction(c: Colour) -> Colour {
    Colour::new(c.x.sqrt(), c.y.sqrt(), c.z.sqrt())
}

pub fn vec_to_rgb(vec: Colour) -> Rgb<u8> {
    Rgb(vec.map(|n| (n.clamp(0.0, 1.0) * 255.0) as u8).into())
}

fn random_vec() -> Vector3 {
    let mut rng = rand::thread_rng();
    Vector3::new(rng.gen(), rng.gen(), rng.gen())
}

fn random_vec_in_sphere() -> Vector3 {
    loop {
        let vec = random_vec();
        if vec.dot(vec) < 1.0 {
            return vec.normalize();
        }
    }
}
fn random_vec_in_hemisphere(normal: Vector3) -> Vector3 {
    let vec = random_vec_in_sphere();
    if vec.dot(normal) > 0.0 {
        vec
    } else {
        -vec
    }
}
