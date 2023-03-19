use std::sync::Arc;

use crate::{
    camera::{Camera, CameraSettings},
    hittable::{Hittable, HittableVec},
    texture::Texture,
    Colour, material::{MaterialManager, Lambertian, Material}, resource_manager::Id,
};

pub struct Scene {
    pub hittable: HittableVec,
    pub cameras: Vec<Camera>,
    pub background: Texture,
    pub materials: MaterialManager
}

impl Default for Scene {
    fn default() -> Self {
        let background: Texture = Colour::new(0.1, 0.65, 0.9).into();
        let default_mat = Arc::new(Lambertian{albedo: Arc::new(background.clone())});
        Self {
            hittable: HittableVec::new(),
            cameras: vec![],
            background,
            materials: MaterialManager::new(default_mat)
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

    pub fn add_material(&mut self, material: Arc<dyn Material>) -> Id<dyn Material> {
        let id = Id::new();
        self.materials.insert(id, material);
        id
    }
}
