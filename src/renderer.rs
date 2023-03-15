use crate::camera::Camera;
use crate::hittable::{HittableVec, Sphere};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
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
        let lambert = Arc::new(Lambertian {
            albedo: Arc::new(Colour::new(0.5, 0.5, 0.8)),
        });
        let metal = Arc::new(Metal {
            albedo: Arc::new(Colour::new(0.8, 0.8, 0.5)),
            fuzz: 0.4,
        });
        let glass = Arc::new(Dielectric {
            refractive_index: 1.5,
        });
        let light = Arc::new(DiffuseLight {
            emit: Arc::new(Colour::new(5.0, 5.0, 5.0)),
        });
        scene.push(Box::new(Sphere::new(point3(-0.5, 0.0, -1.0), 0.5, metal)));
        scene.push(Box::new(Sphere::new(point3(0.5, 0.0, -1.0), 0.5, glass)));
        scene.push(Box::new(Sphere::new(point3(0.0, 2.0, -1.5), 1.0, light)));
        scene.push(Box::new(Sphere::new(
            point3(0.0, -20.5, -1.0),
            20.0,
            lambert,
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
            Colour::new(0.0, 0.0, 0.01),
            50,
        );
        image
    }
}
