use crate::io;

use super::{Colour, rgb_to_vec};




#[derive(Clone)]
pub enum Texture {
    Colour(Colour),
    Image(io::Image),
}

impl Texture {
    pub fn colour_at(&self, u: f64, v: f64) -> Colour {
        match self {
            Self::Colour(c) => c.clone(),
            Self::Image(img) => {
                let (width, height) = img.image().dimensions();
                let mut i = (u.clamp(0.0, 1.0) * (width as f64)) as u32;
                let mut j = (v.clamp(0.0, 1.0) * (height as f64)) as u32;

                i = if i >= width { i - 1 } else { i };
                j = if j >= height { j - 1 } else { j };

                rgb_to_vec(img.image().get_pixel(i, j))
            }
        }
    }
}

impl From<Colour> for Texture {
    fn from(value: Colour) -> Self {
        Texture::Colour(value)
    }
}

impl From<io::Image> for Texture {
    fn from(value: io::Image) -> Self {
        Texture::Image(value)
    }
}

impl Default for Texture {
    fn default() -> Self {
        Texture::Colour([0.5, 0.5, 0.5].into())
    }
}
