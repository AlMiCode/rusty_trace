#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release //

use crate::renderer::Renderer;

use eframe::egui;

mod guielements;
use guielements::*;

#[derive(Clone, Copy)]
pub struct WindowDimensions {
    pub width: u32,
    pub height: u32,
}

pub struct Gui {
    renderer: Renderer,
    elements: Vec<Box<dyn GuiElement>>,
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            elements: vec![],
            renderer: Renderer::new(1280f64 / 720f64),
        }
    }
}

pub fn start(gui: Gui, dimensions: WindowDimensions, title: &str) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(
            dimensions.width as f32,
            dimensions.height as f32,
        )),
        ..Default::default()
    };

    eframe::run_native(title, options, Box::new(|_cc| Box::new(gui)))
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Render").clicked() {
                // implement render function
                self.elements.push(Box::new(ImageGuiElement::new(self.renderer.render((640,360)))));
            }
        });
        for e in &mut self.elements {
            e.show(ctx);
        }
    }
}
