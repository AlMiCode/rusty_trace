use crate::{
    camera::{Camera, CameraSettings},
    hittable::{Hittable, HittableVec},
    texture::Texture,
    Colour,
};

pub struct Scene {
    pub hittable: HittableVec,
    pub cameras: Vec<Camera>,
    pub background: Box<dyn Texture + Send + Sync>,
}

impl Default for Scene {
    fn default() -> Self {
        let background = Box::new(Colour::new(0.0, 0.0, 0.0));
        Self {
            hittable: HittableVec::new(),
            cameras: vec![],
            background,
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
}
