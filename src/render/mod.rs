use std::io::Write;

use crate::{oidn::OIND, vec_repo::VecRepo};
use cgmath::{ElementWise, InnerSpace, Zero};
use hittable::{sphere::Sphere, HittableTrait};
use image::{Rgb, Rgb32FImage};
use material::{Material, MaterialTrait};
use texture::Texture;

use self::scene::{Scene, SceneRef};

pub mod camera;
pub mod hittable;
pub mod material;
pub mod scene;
pub mod texture;

pub type Point3 = cgmath::Point3<f64>;
pub type Vector3 = cgmath::Vector3<f64>;
pub type Colour = cgmath::Vector3<f32>;

pub struct RenderedImage {
    pub colour: Rgb32FImage,
    pub albedo: Rgb32FImage,
    pub normal: Rgb32FImage,
    pub denoised: Rgb32FImage,
}

pub fn render<'a>(dims: (u32, u32), scene: &Scene, sample_count: u32, depth: u32) -> RenderedImage {
    use std::time::Instant;
    let now = Instant::now();

    let SceneRef {
        hittable,
        camera,
        background,
        materials,
        textures,
    } = scene.into();

    let (width, height) = dims;
    let camera = camera.build_with_dimensions(width, height);
    let background = textures.get(*background);

    let mut colour_image = Rgb32FImage::new(width, height);
    let mut albedo_image = Rgb32FImage::new(width, height);
    let mut normal_image = Rgb32FImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let u = x as f64 / (width - 1) as f64;
            let v = y as f64 / (height - 1) as f64;
            let r = camera.get_ray(u, v);
            let (mut colour, albedo, normal) =
                cast_ray_extended(r, hittable, background, materials, textures, depth);
            for _s in 1..sample_count {
                let r = camera.get_ray(u, v);
                colour += cast_ray(r, hittable, background, materials, textures, depth)
            }
            let pixel = Rgb::<f32>(gamma_correction(colour / sample_count as f32).into());
            colour_image.put_pixel(x, height - y - 1, pixel);
            albedo_image.put_pixel(x, height - y - 1, Rgb(albedo.into()));
            normal_image.put_pixel(x, height - y - 1, Rgb(normal.into()));
        }
        print!("\r{}/{} done", y + 1, height);
        std::io::stdout().flush().expect("could not flush stdin");
    }
    println!("\nRendered: {:.2?}", now.elapsed());

    let mut denoised_image = colour_image.clone();
    if OIND.availible() {
        OIND.denoise(
            &mut denoised_image,
            Some(&albedo_image),
            Some(&normal_image),
        );
        println!("Denoised: {:.2?}", now.elapsed());
    }
    RenderedImage {
        colour: colour_image,
        albedo: albedo_image,
        normal: normal_image,
        denoised: denoised_image,
    }
}

pub struct Ray {
    origin: Point3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

pub fn cast_ray(
    ray: Ray,
    hittable: &dyn HittableTrait,
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

fn cast_ray_extended(
    ray: Ray,
    hittable: &dyn HittableTrait,
    background: &Texture,
    materials: &VecRepo<Material>,
    textures: &VecRepo<Texture>,
    depth: u32,
) -> (Colour, Colour, cgmath::Vector3<f32>) {
    if depth == 0 {
        return (Colour::zero(), Colour::zero(), cgmath::Vector3::zero());
    }
    if let Some(hit) = hittable.hit_bounded(&ray, 0.0001, f64::INFINITY) {
        let emitted = materials
            .get(hit.material_id)
            .emit(hit.uv.0, hit.uv.1, textures);
        match materials.get(hit.material_id).scatter(&ray, &hit, textures) {
            None => (emitted, emitted, hit.normal.cast::<f32>().unwrap()),
            Some(scattered) => {
                let next_scattered = cast_ray(
                    scattered.ray,
                    hittable,
                    background,
                    materials,
                    textures,
                    depth - 1,
                );
                (
                    scattered.attenuation.mul_element_wise(next_scattered) + emitted,
                    scattered.attenuation,
                    hit.normal.cast::<f32>().unwrap(),
                )
            }
        }
    } else {
        let (u, v) = Sphere::get_uv(&ray.direction);
        let c = background.colour_at(u, v);
        (c, c, (-ray.direction).cast::<f32>().unwrap().normalize())
    }
}

fn gamma_correction(c: Colour) -> Colour {
    Colour::new(c.x.sqrt(), c.y.sqrt(), c.z.sqrt())
}

fn rgb_to_vec(rgb: &Rgb<u8>) -> Colour {
    Colour::from(rgb.0.map(|n| n as f32 / 255.0))
}

fn random_f64(min: f64, max: f64) -> f64 {
    min + (max - min) * fastrand::f64()
}

fn random_vec() -> Vector3 {
    Vector3::new(
        random_f64(-1.0, 1.0),
        random_f64(-1.0, 1.0),
        random_f64(-1.0, 1.0),
    )
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
