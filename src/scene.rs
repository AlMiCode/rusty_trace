use crate::{
    camera::{Camera, CameraSettings},
    hittable::{Hittable, HittableVec},
    material::{Lambertian, Material, MaterialManager},
    resource_manager::Id,
    texture::{Texture, TextureManager},
    Colour,
};

pub struct Scene {
    pub hittable: HittableVec,
    pub cameras: Vec<Camera>,
    pub background: Id<Texture>,
    pub materials: MaterialManager,
    pub textures: TextureManager,
}

impl Default for Scene {
    fn default() -> Self {
        let background: Texture = Colour::new(0.1, 0.65, 0.9).into();
        let gray: Texture = Colour::new(0.5, 0.5, 0.5).into();
        let mut textures = TextureManager::new(Box::new(gray));
        let id = Id::new();
        textures.insert(id, Box::new(background.clone()));
        let default_mat = Box::new(Lambertian { albedo: id });
        Self {
            hittable: HittableVec::new(),
            cameras: vec![],
            background: id,
            materials: MaterialManager::new(default_mat),
            textures,
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
