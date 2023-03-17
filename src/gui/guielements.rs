use std::sync::Arc;
use std::sync::RwLock;

use crate::{camera::Camera, render, scene::Scene};
use egui::ColorImage;
use egui_extras::RetainedImage;
use image::RgbImage;
use poll_promise::Promise;

pub trait GuiElement {
    fn show(&mut self, ctx: &egui::Context);
}

pub struct ImageGuiElement {
    title: String,
    image: Promise<RetainedImage>,
}

impl ImageGuiElement {
    pub fn new(
        window_id: usize,
        img_dimensions: (u32, u32),
        scene: &Arc<RwLock<Scene>>,
        cam_index: usize,
    ) -> Self {
        let title = format!("Render {window_id}");
        let scene_clone = Arc::clone(scene);
        Self {
            title,
            image: Promise::spawn_thread("debug-renderer", move || {
                let scene_value = scene_clone.read().unwrap();
                let mut image = RgbImage::new(img_dimensions.0, img_dimensions.1);
                render(
                    &mut image,
                    &(*scene_value).cameras[cam_index],
                    &(*scene_value).hittable,
                    &(*scene_value).background,
                    200,
                );
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

pub struct SceneEditor {
    title: String,
    scene: Arc<RwLock<Scene>>,
    sub_elements: Vec<Box<dyn GuiElement>>,
}

impl SceneEditor {
    pub fn new(title: String, scene: Arc<RwLock<Scene>>) -> Self {
        Self {
            title,
            scene,
            sub_elements: vec![],
        }
    }
}

impl GuiElement for SceneEditor {
    fn show(&mut self, ctx: &egui::Context) {
        let pos = egui::pos2(16.0, 128.0);
        let scene_clone = Arc::clone(&self.scene);

        egui::Window::new(&self.title)
            .default_pos(pos)
            .show(ctx, |ui| {
                let reader = scene_clone.read().unwrap();
                if ui.button("Render").clicked() {
                    let guielement = Box::new(ImageGuiElement::new(
                        self.sub_elements.len(),
                        (640, 360),
                        &self.scene,
                        0,
                    ));
                    self.sub_elements.push(guielement);
                }
                let cam_len = reader.cameras.len();
                drop(reader);
                for c in 0..cam_len {
                    let reader = scene_clone.read().unwrap();
                    let mut fov: f64 = reader.cameras[c].settings.fov;
                    if ui.add(egui::DragValue::new(&mut fov)).changed() {
                        println!("Changed");
                        drop(reader);
                        let mut writer = scene_clone.write().unwrap();
                        (*writer).cameras[c].settings.fov = fov;
                        println!("After fov change");
                        (*writer).cameras[c].update();
                        println!("After fov change");
                        println!("After change");
                    }
                }
            });
        for e in &mut self.sub_elements {
            e.show(ctx);
        }
    }
}
