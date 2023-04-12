#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release //

use std::{cell::RefCell, rc::Rc};

use eframe::egui;

mod guielements;
use guielements::*;

use crate::scene::Scene;

#[derive(Clone, Copy)]
pub struct WindowDimensions {
    pub width: u32,
    pub height: u32,
}

pub struct Gui {
    elements: Vec<Box<dyn GuiElement>>,
}

impl Default for Gui {
    fn default() -> Self {
        let scene = Scene::cornell_box();
        /*scene.add_camera(CameraSettings::default());
        scene.add_camera(CameraSettings::default());
        scene.add_camera(CameraSettings::default());

        let blue = scene.add_texture(Box::new(Colour::new(0.8, 0.8, 0.5).into()));
        let gold = scene.add_texture(Box::new(Colour::new(0.8, 0.8, 0.5).into()));
        let white_light = scene.add_texture(Box::new(Colour::new(5.0, 5.0, 5.0).into()));

        let lambert = scene.add_material(Box::new(Lambertian { albedo: blue }));
        let metal = scene.add_material(Box::new(Metal {
            albedo: gold,
            fuzz: 0.4,
        }));
        let glass = scene.add_material(Box::new(Dielectric {
            refractive_index: 1.5,
        }));
        let light = scene.add_material(Box::new(DiffuseLight { emit: white_light }));

        scene.add_shape(Box::new(Sphere::new(point3(-0.5, 0.0, -1.0), 0.5, metal)));
        scene.add_shape(Box::new(Sphere::new(point3(0.5, 0.0, -1.0), 0.5, glass)));
        scene.add_shape(Box::new(Sphere::new(point3(0.0, 2.0, -1.5), 1.0, light)));
        scene.add_shape(Box::new(Sphere::new(
            point3(0.0, -20.5, -1.0),
            20.0,
            lambert,
        )));*/
        let scene_rc = Rc::new(RefCell::new(scene));
        let scene_editor = SceneEditor::new(String::from("Scene editor"), scene_rc);
        Self {
            elements: vec![Box::new(scene_editor)],
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
        // egui::CentralPanel::default().show(ctx, |ui| {

        // });
        for e in &mut self.elements {
            e.show(ctx);
        }
    }
}
