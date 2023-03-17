use std::sync::Arc;
use std::sync::RwLock;

use crate::{render, scene::Scene};
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
        camera_id: usize,
        render_id: usize,
        img_dimensions: (u32, u32),
        scene: &Arc<RwLock<Scene>>,
        cam_index: usize,
    ) -> Self {
        let title = format!("Render {render_id} for camera {camera_id}");
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
        let pos = egui::pos2(10.0, 10.0);
        let scene_clone = Arc::clone(&self.scene);

        egui::Window::new(&self.title)
            .default_pos(pos)
            .vscroll(true)
            .show(ctx, |ui| {
                ui.collapsing("Scene", |ui| {
                    ui.collapsing("Background", |ui| {
                        ui.horizontal(|ui| {
                            let reader = scene_clone.read().unwrap();
                            let mut c = reader.background.as_ref().colour_at(0.0, 0.0);
                            drop(reader);
                            ui.label("R: ");
                            let r = ui.add(egui::DragValue::new(&mut c.x)).changed();
                            ui.label("G: ");
                            let g = ui.add(egui::DragValue::new(&mut c.y)).changed();
                            ui.label("B: ");
                            let b = ui.add(egui::DragValue::new(&mut c.z)).changed();
                            if r || g || b {
                                let mut writer = scene_clone.write().unwrap();
                                (*writer).background.as_mut().set_colour_at(c);
                            }
                        });
                    });
                    ui.collapsing("Objects", |ui| {
                        let reader = scene_clone.read().unwrap();
                        let hittable_len = reader.hittable.len();
                        drop(reader);
                        for i in 0..hittable_len {
                            ui.collapsing(format!("Sphere {i}"), |ui| {
                                ui.label("Position");
                                ui.horizontal(|ui| {
                                    let reader = scene_clone.read().unwrap();
                                    let mut c = reader.hittable[i].as_ref().get_position();
                                    drop(reader);
                                    ui.label("X: ");
                                    let x = ui.add(egui::DragValue::new(&mut c.x)).changed();
                                    ui.label("Y: ");
                                    let y = ui.add(egui::DragValue::new(&mut c.y)).changed();
                                    ui.label("z: ");
                                    let z = ui.add(egui::DragValue::new(&mut c.z)).changed();
                                    if x || y || z {
                                        let mut writer = scene_clone.write().unwrap();
                                        (*writer).hittable[i].as_mut().set_position(c);
                                    }
                                })
                            });
                        }
                    })
                });

                let reader = scene_clone.read().unwrap();
                let cam_len = reader.cameras.len();
                drop(reader);
                for c in 0..cam_len {
                    ui.collapsing(format!("Camera {c}"), |ui| {
                        // Render button
                        {
                            ui.horizontal(|ui| {
                                if ui.button("Render").clicked() {
                                    let guielement = Box::new(ImageGuiElement::new(
                                        c, self.sub_elements.len(),
                                        (640, 360),
                                        &self.scene,
                                        c,
                                    ));
                                    self.sub_elements.push(guielement);
                                }
                            });
                        }
                        // fov dragvalue
                        {
                            ui.horizontal(|ui| {
                                let reader = scene_clone.read().unwrap();
                                let mut fov: f64 = reader.cameras[c].settings.fov;
                                ui.label("Fov:");
                                if ui.add(egui::DragValue::new(&mut fov)).changed() {
                                    drop(reader);
                                    let mut writer = scene_clone.write().unwrap();
                                    (*writer).cameras[c].settings.fov = fov;
                                    (*writer).cameras[c].update();
                                }
                            });
                        }
                        // aperture dragvalue
                        {
                            ui.horizontal(|ui| {
                                let reader = scene_clone.read().unwrap();
                                let mut aperture: f64 = reader.cameras[c].settings.aperture;
                                ui.label("Aperture:");
                                if ui.add(egui::DragValue::new(&mut aperture)).changed() {
                                    drop(reader);
                                    let mut writer = scene_clone.write().unwrap();
                                    (*writer).cameras[c].settings.aperture = aperture;
                                    (*writer).cameras[c].update();
                                }
                            });
                        }
                        // look_at dragvalues
                        {
                            ui.collapsing("Look at", |ui| {
                                let reader = scene_clone.read().unwrap();
                                ui.horizontal(|ui| {
                                    let mut x: f64 = reader.cameras[c].settings.look_at.x;
                                    ui.label("x:");
                                    if ui.add(egui::DragValue::new(&mut x)).changed() {
                                        drop(reader);
                                        let mut writer = scene_clone.write().unwrap();
                                        (*writer).cameras[c].settings.look_at.x = x;
                                        (*writer).cameras[c].update();
                                    }
                                    let reader = scene_clone.read().unwrap();
                                    let mut y: f64 = reader.cameras[c].settings.look_at.y;
                                    ui.label("y:");
                                    if ui.add(egui::DragValue::new(&mut y)).changed() {
                                        drop(reader);
                                        let mut writer = scene_clone.write().unwrap();
                                        (*writer).cameras[c].settings.look_at.y = y;
                                        (*writer).cameras[c].update();
                                    }
                                    let reader = scene_clone.read().unwrap();
                                    let mut z: f64 = reader.cameras[c].settings.look_at.z;
                                    ui.label("z:");
                                    if ui.add(egui::DragValue::new(&mut z)).changed() {
                                        drop(reader);
                                        let mut writer = scene_clone.write().unwrap();
                                        (*writer).cameras[c].settings.look_at.z = z;
                                        (*writer).cameras[c].update();
                                    }
                                })
                            });
                        }
                        {
                            ui.collapsing("Look from", |ui| {
                                let reader = scene_clone.read().unwrap();
                                ui.horizontal(|ui| {
                                    let mut x: f64 = reader.cameras[c].settings.look_from.x;
                                    ui.label("x:");
                                    drop(reader);
                                    if ui.add(egui::DragValue::new(&mut x)).changed() {

                                        let mut writer = scene_clone.write().unwrap();
                                        (*writer).cameras[c].settings.look_from.x = x;
                                        (*writer).cameras[c].update();
                                    }
                                    let reader = scene_clone.read().unwrap();
                                    let mut y: f64 = reader.cameras[c].settings.look_from.y;
                                    drop(reader);
                                    ui.label("y:");
                                    if ui.add(egui::DragValue::new(&mut y)).changed() {
                                        let mut writer = scene_clone.write().unwrap();
                                        (*writer).cameras[c].settings.look_from.y = y;
                                        (*writer).cameras[c].update();
                                    }
                                    let reader = scene_clone.read().unwrap();
                                    let mut z: f64 = reader.cameras[c].settings.look_from.z;
                                    drop(reader);
                                    ui.label("z:");
                                    if ui.add(egui::DragValue::new(&mut z)).changed() {
                                        let mut writer = scene_clone.write().unwrap();
                                        (*writer).cameras[c].settings.look_from.z = z;
                                        (*writer).cameras[c].update();
                                    }
                                })
                            });
                        }
                        ui.separator();
                    });
                }
            });
        for e in &mut self.sub_elements {
            e.show(ctx);
        }
    }
}
