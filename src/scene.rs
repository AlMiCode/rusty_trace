use std::{cell::RefCell, default, sync::Arc};

use cgmath::{point3, vec3, Point3};
use image::RgbImage;

use crate::{
    camera::{Camera, CameraSettings},
    hittable::{Hittable, HittableVec, Plane, Rect},
    material::{DiffuseLight, Lambertian, Material},
    repo::{ARepo, Id, Repo},
    texture::Texture,
    Colour,
};

#[derive(Clone)]
pub struct Scene {
    pub hittable: HittableVec,
    pub cameras: Vec<Camera>,
    pub background: Id<Texture>,
    pub materials: Repo<dyn Material>,
    pub textures: RefCell<Repo<Texture>>,
    pub images: RefCell<ARepo<RgbImage>>,
}

impl Default for Scene {
    fn default() -> Self {
        let background: Texture = Colour::new(0.1, 0.65, 0.9).into();
        let gray: Texture = Colour::new(0.5, 0.5, 0.5).into();
        let mut textures = Repo::new(Box::new(gray));
        let id = textures.insert(Box::new(background.clone()));
        let default_mat = Box::new(Lambertian { albedo: id });

        let default_img = Arc::new(RgbImage::new(4, 4));
        Self {
            hittable: HittableVec::new(),
            cameras: vec![],
            background: id,
            materials: Repo::new(default_mat),
            textures: RefCell::new(textures),
            images: RefCell::new(ARepo::new(default_img)),
        }
    }
}

impl Scene {
    pub fn add_shape(&mut self, shape: Box<dyn Hittable + Sync + Send>) {
        self.hittable.push(shape);
    }

    pub fn add_camera(&mut self, settings: CameraSettings) {
        self.cameras.push(Camera::new(settings))
    }

    pub fn add_material(&mut self, material: Box<dyn Material>) -> Id<dyn Material> {
        self.materials.insert(material)
    }

    pub fn add_texture(&self, texture: Box<Texture>) -> Id<Texture> {
        self.textures.borrow_mut().insert(texture)
    }

    pub fn add_image(&self, image: Arc<RgbImage>) -> Id<RgbImage> {
        self.images.borrow_mut().insert(image)
    }

    pub fn cornell_box() -> Self {
        // Use this for testing.
        let default: Texture = Colour::new(0.5, 0.5, 0.5).into();
        let mut textures = Repo::<Texture>::new(Box::new(default));

        let red_tex = textures.insert(Box::new(Colour::new(0.65, 0.05, 0.05).into()));
        let white_tex = textures.insert(Box::new(Colour::new(0.73, 0.73, 0.73).into()));
        let green_tex = textures.insert(Box::new(Colour::new(0.12, 0.45, 0.15).into()));
        let light_tex = textures.insert(Box::new(Colour::new(1.0, 1.0, 1.0).into()));

        let default = Box::new(Lambertian { albedo: white_tex }); // TODO: Do something with 'default's. They are tedious.
        let mut materials = Repo::<dyn Material>::new(default);

        let red_mat = materials.insert(Box::new(Lambertian { albedo: red_tex }));
        let white_mat = materials.insert(Box::new(Lambertian { albedo: white_tex }));
        let green_mat = materials.insert(Box::new(Lambertian { albedo: green_tex }));
        let light_mat = materials.insert(Box::new(DiffuseLight {
            emit: light_tex,
            amplify: 15f32,
        }));

        let green_wall = Rect::new(
            &point3(555.0, 0.0, 0.0),
            &point3(555.0, 555.0, 555.0),
            Plane::YZ,
            green_mat,
        );
        let red_wall = Rect::new(
            &point3(0.0, 0.0, 0.0),
            &point3(0.0, 555.0, 555.0),
            Plane::YZ,
            red_mat,
        );
        let floor = Rect::new(
            &point3(0.0, 0.0, 0.0),
            &point3(555.0, 0.0, 555.0),
            Plane::XZ,
            white_mat,
        );
        let ceiling = Rect::new(
            &point3(0.0, 555.0, 0.0),
            &point3(555.0, 555.0, 555.0),
            Plane::XZ,
            white_mat,
        );
        let back_wall = Rect::new(
            &point3(0.0, 0.0, 555.0),
            &point3(555.0, 555.0, 555.0),
            Plane::XY,
            white_mat,
        );
        let light_source = Rect::new(
            &point3(213.0, 554.0, 227.0),
            &point3(343.0, 554.0, 332.0),
            Plane::XZ,
            light_mat,
        );

        let default_img = Arc::new(RgbImage::new(4, 4));
        Self {
            hittable: vec![
                Box::new(green_wall),
                Box::new(red_wall),
                Box::new(floor),
                Box::new(ceiling),
                Box::new(back_wall),
                Box::new(light_source),
            ],
            cameras: vec![Camera::new(CameraSettings {
                look_from: point3(278.0, 278.0, -800.0),
                look_at: point3(278.0, 278.0, 0.0),
                fov: 40.0,
                aperture: 1.0 / 16.0,
                ..Default::default()
            })],
            background: white_tex,
            materials,
            textures: textures.into(),
            images: ARepo::new(default_img).into(),
        }
    }
}
