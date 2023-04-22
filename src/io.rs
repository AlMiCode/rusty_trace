use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::Arc,
};

pub fn load_image(filename: String) -> Option<image::RgbImage> {
    try_load_image(&filename.into()).ok()
}

pub fn try_load_image(filename: &std::path::PathBuf) -> image::ImageResult<image::RgbImage> {
    Ok(image::io::Reader::open(filename)?.decode()?.to_rgb8())
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

    pub fn try_open(filename: &std::path::PathBuf) -> Result<Self, image::ImageError> {
        Ok(Self::new(
            image::io::Reader::open(filename)?.decode()?.into_rgb8(),
        ))
    }

    pub fn image(&self) -> &image::RgbImage {
        &self.image
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
