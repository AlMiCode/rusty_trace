#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release //

use std::{cell::RefCell, rc::Rc};

use eframe::egui;

mod guielements;
use guielements::*;

mod views;

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
        let scene_rc = Rc::new(RefCell::new(scene));
        let scene_editor = SceneEditor::new(String::from("Scene editor"), scene_rc);
        Self {
            elements: vec![Box::new(scene_editor)],
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
        // egui::CentralPanel::default().show(ctx, |ui| {

        // });
        for e in &mut self.elements {
            e.show(ctx);
        }
    }
}
