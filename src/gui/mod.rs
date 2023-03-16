#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release //

use crate::{scene::Scene, camera::CameraSettings, hittable::Sphere, material::{DiffuseLight, Dielectric, Metal, Lambertian}, Colour};

use std::sync::Arc;
use egui::mutex::RwLock;

use cgmath::point3;
use eframe::egui;

mod guielements;
use guielements::*;

#[derive(Clone, Copy)]
pub struct WindowDimensions {
    pub width: u32,
    pub height: u32,
}

pub struct Gui {
    elements: Vec<Box<dyn GuiElement>>,
    scenes: Vec<Arc<RwLock<Scene>>>,
}

impl Default for Gui {
    fn default() -> Self {
        let mut scene = Scene::default();
        scene.add_camera(
            CameraSettings::default(),
        );
        let lambert = Arc::new(Lambertian {
            albedo: Arc::new(Colour::new(0.5, 0.5, 0.8)),
        });
        let metal = Arc::new(Metal {
            albedo: Arc::new(Colour::new(0.8, 0.8, 0.5)),
            fuzz: 0.4,
        });
        let glass = Arc::new(Dielectric {
            refractive_index: 1.5,
        });
        let light = Arc::new(DiffuseLight {
            emit: Arc::new(Colour::new(5.0, 5.0, 5.0)),
        });
        scene.add_shape(Box::new(Sphere::new(point3(-0.5, 0.0, -1.0), 0.5, metal)));
        scene.add_shape(Box::new(Sphere::new(point3(0.5, 0.0, -1.0), 0.5, glass)));
        scene.add_shape(Box::new(Sphere::new(point3(0.0, 2.0, -1.5), 1.0, light)));
        scene.add_shape(Box::new(Sphere::new(
            point3(0.0, -20.5, -1.0),
            20.0,
            lambert,
        )));
        Self {
            elements: vec![],
            scenes: vec![Arc::new(RwLock::new(scene))]
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
<<<<<<< Updated upstream
                // implement render function
                self.elements.push(Box::new(ImageGuiElement::new(self.elements.len(), self.renderer.render((640,360)))));
=======
                let guielement = Box::new(ImageGuiElement::new(
                    self.elements.len(),
                    (640,360),
                    &self.scenes[0],
                    0
                ));
                self.elements.push(guielement);
>>>>>>> Stashed changes
            }
        });
        for e in &mut self.elements {
            e.show(ctx);
        }
    }
}
