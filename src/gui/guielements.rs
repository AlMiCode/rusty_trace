use std::sync::Arc;

use egui::{ColorImage, mutex::RwLock};
use egui_extras::RetainedImage;
use image::RgbImage;
use poll_promise::Promise;
use crate::{scene::Scene, render};

pub trait GuiElement {
    fn show(&mut self, ctx: &egui::Context);
}

pub struct ImageGuiElement {
    title: String,
    image: Promise<RetainedImage>,
}

impl ImageGuiElement {
    pub fn new(window_id: usize, img_dimensions: (u32, u32), scene: &Arc<RwLock<Scene>>, cam_index: usize) -> Self {
        let title = format!("Render {window_id}");
        let scene_clone = Arc::clone(scene);
        Self {
            title,
            image: Promise::spawn_thread("debug-renderer", move || {
                let scene_value = scene_clone.read();
                let mut image = RgbImage::new(img_dimensions.0, img_dimensions.1);
                render(&mut image, &(*scene_value).cameras[cam_index], &(*scene_value).hittable, &(*scene_value).background, 50);
                RetainedImage::from_color_image(
                    "render",
                    ColorImage::from_rgb(
                        [image.width() as usize, image.height() as usize],
                        image.as_raw(),
                    ),
                )
            }),
        }
    }
}

impl GuiElement for ImageGuiElement {
    fn show(&mut self, ctx: &egui::Context) {
        let pos = egui::pos2(16.0, 128.0);
        egui::Window::new(&self.title)
            .default_pos(pos)
            .show(ctx, |ui| match self.image.ready() {
                None => ui.spinner(),
                Some(image) => image.show(ui),
            });
    }
}
