use crate::{
    camera::{Camera, CameraSettings},
    hittable::{Hittable, HittableVec},
    texture::Texture,
    Colour,
};

pub struct Scene {
    pub hittable: HittableVec,
    pub cameras: Vec<Camera>,
    pub background: Texture,
}

impl Default for Scene {
    fn default() -> Self {
        let background = Colour::new(0.1, 0.65, 0.9).into();
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
