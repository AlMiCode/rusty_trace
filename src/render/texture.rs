use std::{sync::Arc, collections::hash_map::DefaultHasher, hash::{Hash, Hasher}, ops::Deref};

use super::{rgb_to_vec, Colour};

#[derive(Clone, PartialEq)]
pub enum Texture {
    Colour(Colour),
    Image(Image),
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

impl From<Image> for Texture {
    fn from(value: Image) -> Self {
        Texture::Image(value)
    }
}

impl Default for Texture {
    fn default() -> Self {
        Texture::Colour([0.5, 0.5, 0.5].into())
    }
}

#[derive(Clone, Eq)]
pub struct Image {
    image: Arc<image::RgbImage>,
    hash: u64,
}

impl Image {
    pub fn new(image: image::RgbImage) -> Self {
        let mut hasher = DefaultHasher::new();
        image.hash(&mut hasher);
        Self {
            image: Arc::new(image),
            hash: hasher.finish(),
        }
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
    fn ne(&self, other: &Self) -> bool {
        self.hash != other.hash
    }
}

impl Hash for Image {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash);
    }
}

impl Deref for Image {
    type Target = image::RgbImage;

    fn deref(&self) -> &Self::Target {
        &self.image
    }
}
