#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release //

use eframe::egui;

mod guielements;
use guielements::*;

mod image_storage;
mod logger;

mod views;

use ray::render::scene::Scene;

pub struct Gui {
    elements: Vec<Box<dyn GuiElement>>,
}

impl Default for Gui {
    fn default() -> Self {
        let scene = Scene::cornell_box();
        Self {
            elements: vec![Box::new(ProjectEditor::from_scene(scene))],
        }
    }
}

impl Gui {
    pub fn start(self, title: &str) -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
            ..Default::default()
        };

        eframe::run_native(title, options, Box::new(|_cc| Box::new(self)))
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for e in &mut self.elements {
            e.show(ctx);
        }
    }
}
