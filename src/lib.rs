use cgmath::{ElementWise, InnerSpace, Zero};
use hittable::{Hittable, Sphere};
use image::{DynamicImage, Rgb, Rgb32FImage, RgbImage};
use material::{Material, MaterialTrait};
use oidn::OpenImageDenoise;
use repo::VecRepo;
use scene::Scene;
use texture::Texture;

use std::io::Write;

pub mod camera;
pub mod gui;
pub mod hittable;
mod io;
pub mod material;
pub mod oidn;
mod repo;
pub mod scene;
pub mod texture;

pub type Point3 = cgmath::Point3<f64>;
pub type Vector3 = cgmath::Vector3<f64>;
pub type Colour = cgmath::Vector3<f32>;

pub fn render(image: &mut RgbImage, scene: &Scene, sample_count: u32, depth: u32) {
    use std::time::Instant;
    let now = Instant::now();

    let Scene {
        hittable,
        camera,
        background,
        materials,
        textures,
    } = scene;

    let (width, height) = image.dimensions();
    let camera = camera.build_with_dimensions(width, height);
    let background = textures.get(*background);

    for y in 0..height {
        for x in 0..width {
            let mut colour = Colour::zero();
            for _s in 0..sample_count {
                let u = x as f64 / (width - 1) as f64;
                let v = y as f64 / (height - 1) as f64;
                let r = camera.get_ray(u, v);

                colour += cast_ray(r, hittable, background, materials, textures, depth)
            }
            let pixel: Rgb<u8> = vec_to_rgb(gamma_correction(colour / sample_count as f32));
            image.put_pixel(x, height - y - 1, pixel);
        }
        print!("\r{}/{} done", y + 1, height);
        std::io::stdout().flush().expect("could not flush stdin");
    }
    println!("\nElapsed: {:.2?}", now.elapsed());
    *image = denoise(image.clone());
}

fn denoise(image: RgbImage) -> RgbImage {
    let image = DynamicImage::ImageRgb8(image).into_rgb32f();
    let mut output = image.as_raw().clone();

    OpenImageDenoise::denoise(
        image.as_raw(),
        (image.width() as usize, image.height() as usize),
        &mut output,
    );
    let output = Rgb32FImage::from_vec(image.width(), image.height(), output).unwrap();
    DynamicImage::ImageRgb32F(output).into_rgb8()
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

pub fn cast_ray(
    ray: Ray,
    hittable: &dyn Hittable,
    background: &Texture,
    materials: &VecRepo<Material>,
    textures: &VecRepo<Texture>,
    depth: u32,
) -> Colour {
    if depth == 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }
    if let Some(hit) = hittable.hit_bounded(&ray, 0.0001, f64::INFINITY) {
        let emitted = materials
            .get(hit.material_id)
            .emit(hit.uv.0, hit.uv.1, textures);
        match materials.get(hit.material_id).scatter(&ray, &hit, textures) {
            None => emitted,
            Some(scattered) => {
                scattered.attenuation.mul_element_wise(cast_ray(
                    scattered.ray,
                    hittable,
                    background,
                    materials,
                    textures,
                    depth - 1,
                )) + emitted
            }
        }
    } else {
        let (u, v) = Sphere::get_uv(&ray.direction);
        background.colour_at(u, v)
    }
}

fn gamma_correction(c: Colour) -> Colour {
    Colour::new(c.x.sqrt(), c.y.sqrt(), c.z.sqrt())
}

pub fn vec_to_rgb(vec: Colour) -> Rgb<u8> {
    Rgb(vec.map(|n| (n.clamp(0.0, 1.0) * 255.0) as u8).into())
}

pub fn rgb_to_vec(rgb: &Rgb<u8>) -> Colour {
    Colour::from(rgb.0.map(|n| n as f32 / 255.0))
}

fn random_f64() -> f64 {
    fastrand::f64() * 2.0 - 1.0
}

fn random_vec() -> Vector3 {
    Vector3::new(random_f64(), random_f64(), random_f64())
}

fn random_vec_in_sphere() -> Vector3 {
    loop {
        let vec = random_vec();
        if vec.magnitude2() < 1.0 {
            return vec.normalize();
        }
    }
}

fn random_vec_in_disc() -> Vector3 {
    loop {
        let mut vec = random_vec();
        vec.z = 0.0;
        if vec.magnitude2() < 1.0 {
            return vec.normalize();
        }
    }
}
