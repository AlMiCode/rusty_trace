use std::{collections::HashMap, sync::{Mutex}};

use egui::ColorImage;
use egui_extras::RetainedImage;
use lazy_static::lazy_static;

use crate::render::texture::Image;

lazy_static!(
    pub static ref IMAGE_STORAGE: ImageStorage = ImageStorage::default();
);

#[derive(Default)]
pub struct ImageStorage {
    data: Mutex<HashMap<Image, RetainedImage>>
}

impl ImageStorage {
    pub fn add_retained(&self, img: &Image) {
        self.with_retained(img, |_x|{});
    }

    pub fn with_retained(&self, img: &Image, f: impl FnOnce(&RetainedImage)) {
        self.data.lock().and_then(|mut data|{
            if !data.contains_key(img) {
                data.insert(img.clone(), self.image_to_retained(img));
            }
            let rtimg = data.get(img).expect("RetainedImage MUST be there");
            Ok(f(rtimg))
        }).expect("ImageStorage MUST never fail");
    }

    fn image_to_retained(&self, image: &image::RgbImage) -> RetainedImage {
        RetainedImage::from_color_image(
            "opened file",
            ColorImage::from_rgb(
                [image.width() as usize, image.height() as usize],
                image.as_raw(),
            ),
        )
    }
}