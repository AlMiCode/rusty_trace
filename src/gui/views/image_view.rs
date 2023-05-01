use std::sync::mpsc::Receiver;

use super::{image_to_retained, View};
use egui::Ui;
use egui_extras::RetainedImage;
use image::RgbImage;

pub struct ImageView {
    title: String,
    rx: Receiver<RgbImage>,
    image: Option<RetainedImage>,
}

impl ImageView {
    pub fn new(title: impl Into<String>, rx: Receiver<RgbImage>) -> Self {
        Self {
            title: title.into(),
            rx,
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
            match self.rx.try_recv() {
                Ok(img) => self.image = Some(image_to_retained(&img)),
                Err(_) => {}
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
