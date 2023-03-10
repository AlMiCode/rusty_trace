use crate::gui::WindowDimensions;

use super::shapes::Shape;
use super::Camera;

pub struct Scene {
    camera: Camera,
    dimensions: WindowDimensions,
    shapes: Vec<Shape>,
}

impl Scene {
    pub fn new(camera: Camera, dimensions: WindowDimensions) -> Scene {
        Scene {
            camera,
            dimensions,
            shapes: Vec::new(),
        }
    }

    pub fn render(&mut self, buffer: &mut [u8], pitch: usize) {
        // this just to test the gui
        for n in 0..256 as usize {
            let offset = n * pitch + n * 3;
            buffer[offset] = 0xFF;
            buffer[offset + 1] = 0x0;
            buffer[offset + 2] = 0x0;
        }
    }
}
