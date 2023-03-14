use crate::camera::Camera;
use crate::hittable::{HittableVec, Sphere};
use crate::material::Lambertian;
use crate::{render, Colour};
use cgmath::point3;
use image::RgbImage;
use std::sync::Arc;

// temporary code for gui to call in order to get a rendered image on buttonpress

pub struct Renderer {
    camera: Camera,
    scene: HittableVec,
}

impl Renderer {
    pub fn new(/*tmp*/ aspect_ratio: f64) -> Renderer {
        let camera = Camera::from_aspect_ratio(aspect_ratio);
        let mut scene = HittableVec::new();
        let material = Arc::new(Lambertian {
            albedo: Colour::new(0.8, 0.8, 0.8),
        });
        scene.push(Box::new(Sphere::new(
            point3(0.0, 0.0, -1.0),
            0.5,
            material.clone(),
        )));
        scene.push(Box::new(Sphere::new(
            point3(0.0, -20.5, -1.0),
            20.0,
            material,
        )));

        Renderer { camera, scene }
    }

    pub fn render(&self, size: (u32, u32)) -> image::RgbImage {
        let (width, height) = size;
        let mut image = RgbImage::new(width, height);
        render(
            &mut image,
            &self.camera,
            &self.scene,
            Colour::new(0.8, 0.8, 0.9),
            50,
        );
        image
    }
}
