use image::RgbImage;

use crate::{Colour, rgb_to_vec};

trait Texture {
    fn colour_at(&self, u: f64, v: f64) -> Colour;
}

impl Texture for Colour {
    fn colour_at(&self, u: f64, v: f64) -> Colour { self.clone() }
}

impl Texture for RgbImage {
    fn colour_at(&self, u: f64, v: f64) -> Colour {
        let (width, height) = self.dimensions();
        let mut i = (u.clamp(0.0, 1.0) * (width as f64)) as u32;
        let mut j = (v.clamp(0.0, 1.0) * (height as f64)) as u32;

        i = if i >= width { i-1 } else {i};
        j = if j >= height { j-1 } else {j};

        rgb_to_vec(self.get_pixel(i, j))
    }
}