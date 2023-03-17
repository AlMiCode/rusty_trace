use image::RgbImage;

use crate::{rgb_to_vec, Colour};

pub trait Texture {
    fn colour_at(&self, u: f64, v: f64) -> Colour;
    fn set_colour_at(&mut self, c: Colour);
}

impl Texture for Colour {
    fn colour_at(&self, _u: f64, _v: f64) -> Colour {
        self.clone()
    }

    fn set_colour_at(&mut self, c: Colour) {
        *self = c;
    }
}

impl Texture for RgbImage {
    fn colour_at(&self, u: f64, v: f64) -> Colour {
        let (width, height) = self.dimensions();
        let mut i = (u.clamp(0.0, 1.0) * (width as f64)) as u32;
        let mut j = (v.clamp(0.0, 1.0) * (height as f64)) as u32;

        i = if i >= width { i - 1 } else { i };
        j = if j >= height { j - 1 } else { j };

        rgb_to_vec(self.get_pixel(i, j))
    }

    fn set_colour_at(&mut self, c: Colour) {
        unimplemented!();
    }
}
