use crate::{
    camera::{Camera, CameraSettings},
    hittable::{Hittable, HittableVec},
    texture::Texture,
    Colour, material::{Lambertian, Material}, repo::{Id, Repo},
};

#[derive(Clone)]
pub struct Scene {
    pub hittable: HittableVec,
    pub cameras: Vec<Camera>,
    pub background: Id<Texture>,
    pub materials: Repo<dyn Material>,
    pub textures: Repo<Texture>
}

impl Default for Scene {
    fn default() -> Self {
        let background: Texture = Colour::new(0.1, 0.65, 0.9).into();
        let gray: Texture = Colour::new(0.5, 0.5, 0.5).into();
        let mut textures = Repo::new(Box::new(gray));
        let id = Id::new();
        textures.insert(id, Box::new(background.clone()));
        let default_mat = Box::new(Lambertian{albedo: id});
        Self {
            hittable: HittableVec::new(),
            cameras: vec![],
            background: id,
            materials: Repo::new(default_mat),
            textures
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
        let id = Id::new();
        self.materials.insert(id, material);
        id
    }

    pub fn add_texture(&mut self, texture: Box<Texture>) -> Id<Texture> {
        let id = Id::new();
        self.textures.insert(id, texture);
        id
    }
}
