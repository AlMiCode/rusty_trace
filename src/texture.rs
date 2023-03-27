use image::RgbImage;

use crate::{rgb_to_vec, Colour, repo::{Id, ARepo}};

#[derive(Clone)]
pub enum Texture {
    Colour(Colour),
    Image(Id<RgbImage>),
}

impl Texture {
    pub fn colour_at(&self, u: f64, v: f64, images: &ARepo<RgbImage>) -> Colour {
        match self {
            Self::Colour(c) => c.clone(),
            Self::Image(img_id) => {
                let img = images.get(*img_id);
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

impl From<Id<RgbImage>> for Texture {
    fn from(value: Id<RgbImage>) -> Self {
        Texture::Image(value)
    }
}
