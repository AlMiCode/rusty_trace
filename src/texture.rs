use image::RgbImage;

use crate::{rgb_to_vec, Colour};

pub trait Texture2 {
    fn colour_at(&self, u: f64, v: f64) -> Colour;
}

impl Texture2 for Colour {
    fn colour_at(&self, _u: f64, _v: f64) -> Colour {
        self.clone()
    }
}

impl Texture2 for RgbImage {
    fn colour_at(&self, u: f64, v: f64) -> Colour {
        let (width, height) = self.dimensions();
        let mut i = (u.clamp(0.0, 1.0) * (width as f64)) as u32;
        let mut j = (v.clamp(0.0, 1.0) * (height as f64)) as u32;

        i = if i >= width { i - 1 } else { i };
        j = if j >= height { j - 1 } else { j };

        rgb_to_vec(self.get_pixel(i, j))
    }
}

#[derive(Clone)]
pub enum Texture {
    Colour(Colour),
    Image(RgbImage),
}

impl Texture {
    pub fn colour_at(&self, u: f64, v: f64) -> Colour {
        match self {
            Self::Colour(c) => c.clone(),
            Self::Image(img) => {
                let (width, height) = img.dimensions();
                let mut i = (u.clamp(0.0, 1.0) * (width as f64)) as u32;
                let mut j = (v.clamp(0.0, 1.0) * (height as f64)) as u32;

                i = if i >= width { i - 1 } else { i };
                j = if j >= height { j - 1 } else { j };

                rgb_to_vec(img.get_pixel(i, j))
            }
        }
    }
}

impl From<Colour> for Texture {
    fn from(value: Colour) -> Self {
        Texture::Colour(value)
    }
}

impl From<RgbImage> for Texture {
    fn from(value: RgbImage) -> Self {
        Texture::Image(value)
    }
}