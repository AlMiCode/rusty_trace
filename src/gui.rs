#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui_extras::RetainedImage;
use egui::ColorImage;
use image::RgbImage;

#[derive(Clone, Copy)]
pub struct WindowDimensions {
    pub width: u32,
    pub height: u32,
}

pub struct Gui {
    images: Vec<RetainedImage>,
}

impl Default for Gui {
    fn default() -> Self {
        Self { images: vec![] }
    }
}

impl Gui {
    pub fn add_image(&mut self, image: RgbImage) -> Result<(), String> {
        self.images.push(RetainedImage::from_color_image(
            "render",
            ColorImage::from_rgb([image.dimensions().0 as usize, image.dimensions().1 as usize], image.as_raw())
        ));
        Ok(())
    }
}

pub fn start(gui: Gui, dimensions: WindowDimensions, title: &str) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(dimensions.width as f32, dimensions.height as f32)),
        ..Default::default()
    };

    eframe::run_native(
        title,
        options,
        Box::new(|_cc| Box::new(gui)),
    )
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Images:");
            for image in &self.images {
                image.show(ui);
            }
            if ui.button("Render!").clicked() {
                println!("Render!");
            }
        });
    }
}
