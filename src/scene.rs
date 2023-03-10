use crate::shapes::ShapeEnum;

use crate::gui::WindowDimensions;
use crate::Camera;
use crate::{ray_colour, vec_to_rgb, Ray};

use image::Rgb;

pub struct Scene {
    camera: Camera,
    dimensions: WindowDimensions,
    shapes: Vec<ShapeEnum>,
}

impl Scene {
    pub fn new(camera: Camera, dimensions: WindowDimensions) -> Scene {
        Scene {
            camera,
            dimensions,
            shapes: Vec::new(),
        }
    }

    pub fn add_shape(&mut self, shape: ShapeEnum) {
        self.shapes.push(shape);
    }

    pub fn render(&mut self, buffer: &mut [u8], pitch: usize) {
        for j in 0..self.dimensions.height {
            for i in 0..self.dimensions.width {
                let u: f64 = i as f64 / (self.dimensions.width - 1) as f64;
                let v: f64 = j as f64 / (self.dimensions.height - 1) as f64;
                let r = Ray::new(
                    self.camera.origin,
                    self.camera.lower_left_corner
                        + u * self.camera.horizontal
                        + v * self.camera.vertical
                        - self.camera.origin,
                );
                let rgb: Rgb<u8> = vec_to_rgb(ray_colour(r, &self.shapes));
                let offset = j as usize * pitch + i as usize * 3;
                buffer[offset] = rgb[0];
                buffer[offset + 1] = rgb[1];
                buffer[offset + 2] = rgb[2];
            }
        }
    }
}
