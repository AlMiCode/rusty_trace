use super::{image_to_retained, View};
use egui::Ui;
use egui_extras::RetainedImage;
use image::RgbImage;
use poll_promise::Promise;

pub struct ImageView {
    title: String,
    promise: Promise<RgbImage>,
    image: Option<RetainedImage>,
}

impl ImageView {
    pub fn new(title: impl Into<String>, promise: Promise<RgbImage>) -> Self {
        Self {
            title: title.into(),
            promise,
            image: None,
        }
    }
}

impl View for ImageView {
    fn title(&self) -> &str {
        &self.title
    }

    fn ui(&mut self, ui: &mut Ui) {
        let size = if let Some(ref img) = self.image {
            img.show_max_size(ui, ui.available_size());
            Some(img.size())
        } else {
            if let Some(img) = self.promise.ready() {
                self.image = Some(image_to_retained(img));
            }
            ui.spinner();
            None
        };
        ui.horizontal(|ui| {
            ui.label(&self.title);
            if let Some(size) = size {
                ui.label(format!("Size: {}x{}", size[0], size[1]));
            }
            if ui.button("Save").clicked() {
                eprintln!("Save image");
            }
        });
    }
}
