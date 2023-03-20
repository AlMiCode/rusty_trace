use std::sync::Arc;
use std::sync::RwLock;

use crate::resource_manager::Id;
use crate::texture::Texture;
use crate::{render, scene::Scene};
use egui::color_picker;
use egui::Color32;
use egui::ColorImage;
use egui_extras::RetainedImage;
use image::RgbImage;
use poll_promise::Promise;

pub trait GuiElement {
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
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
                    &(*scene_value).textures.get((*scene_value).background),
                    &(*scene_value).materials,
                    &(*scene_value).textures,
                    5,
                    30,
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
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        let pos = egui::pos2(16.0, 128.0);
        egui::Window::new(&self.title)
            .default_pos(pos)
            .open(open)
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
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        let pos = egui::pos2(10.0, 10.0);
        let scene_clone = Arc::clone(&self.scene);

        egui::Window::new(&self.title)
            .default_pos(pos)
            .open(open)
            .vscroll(true)
            .show(ctx, |ui| {
                ui.collapsing("Scene", |ui| {
                    ui.collapsing("Background", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Current: ");
                            let reader = scene_clone.read().unwrap();
                            let tex = reader.background;
                            if let Texture::Colour(c) = reader.textures.get(tex) {
                                let colour: Color32 = egui::Rgba::from_rgb(c.x, c.y, c.z).into();
                                color_picker::show_color(ui, colour, egui::vec2(35.0, 15.0));
                            } else {
                                ui.label("Image. Unimplemented");
                            }
                            drop(reader);
                            if ui.button("Change").clicked() {
                                let tex_editor = Box::new(TexturesEditor::new(scene_clone.clone()));
                                self.sub_elements.push(tex_editor);
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
                                        c,
                                        self.sub_elements.len(),
                                        (400, 400),
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
        // FIXME: Issues with length not updating after element is removed. sometimes causes crash when closing windows
        for i in 0..self.sub_elements.len() {
            let mut is_open = true;
            println!("index: {}, length: {}", i, self.sub_elements.len());
            self.sub_elements[i].show(ctx, &mut is_open);
            if !is_open {
                self.sub_elements.remove(i);
            }
        }
    }
}

struct TextureEditor {
    tex_id: Id<Texture>,
    rgb: [f32; 3],
    image: RgbImage,
    choosing_colour: bool,
    scene_handle: Arc<RwLock<Scene>>,
}

impl TextureEditor {
    fn new(tex_id: Id<Texture>, scene_handle: Arc<RwLock<Scene>>) -> Self {
        TextureEditor {
            tex_id,
            rgb: [0f32, 0f32, 0f32],
            image: RgbImage::new(8, 8),
            choosing_colour: true,
            scene_handle,
        }
    }
}

impl GuiElement for TextureEditor {
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        let pos = egui::pos2(10.0, 10.0);

        egui::Window::new(format!("Texture {}", self.tex_id))
            .default_pos(pos)
            .open(open)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.choosing_colour, true, "Colour");
                    ui.selectable_value(&mut self.choosing_colour, false, "Image");
                });
                ui.horizontal(|ui| {
                    if self.choosing_colour {
                        egui::color_picker::color_edit_button_rgb(ui, &mut self.rgb);
                        if ui.button("Set").clicked() {
                            let mut writer = self.scene_handle.write().unwrap();
                            (*(*writer).textures.get_mut(self.tex_id)) =
                                Texture::Colour(self.rgb.into());
                        }
                    } else {
                        ui.label("File picker. Unimplemented");
                    }
                });
            });
    }
}

struct TexturesEditor {
    scene_handle: Arc<RwLock<Scene>>,
    sub_elements: Vec<Box<dyn GuiElement>>,
}

impl TexturesEditor {
    fn new(scene_handle: Arc<RwLock<Scene>>) -> Self {
        TexturesEditor {
            scene_handle,
            sub_elements: vec![],
        }
    }
}

impl GuiElement for TexturesEditor {
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new("Textures Editor")
            .open(open)
            .show(ctx, |ui| {
                let reader = self.scene_handle.read().unwrap();

                ui.collapsing("Default", |ui| {
                    ui.horizontal(|ui| {
                        if let Texture::Colour(c) = reader.textures.get_default() {
                            ui.label("Colour: ");
                            let colour: Color32 = egui::Rgba::from_rgb(c.x, c.y, c.z).into();
                            color_picker::show_color(ui, colour, egui::vec2(35.0, 15.0));
                        } else {
                            ui.label("Image: Unimplemented");
                        }
                    });
                });

                for (id, tex) in reader.textures.iter() {
                    ui.collapsing(format!("Texture {}", id), |ui| {
                        ui.horizontal(|ui| {
                            if let Texture::Colour(c) = tex.as_ref() {
                                ui.label("Colour: ");
                                let colour: Color32 = egui::Rgba::from_rgb(c.x, c.y, c.z).into();
                                color_picker::show_color(ui, colour, egui::vec2(35.0, 15.0));
                            } else {
                                ui.label("Image: Unimplemented");
                            }
                            if ui.button("Change").clicked() {
                                let tex_editor =
                                    Box::new(TextureEditor::new(*id, self.scene_handle.clone()));
                                self.sub_elements.push(tex_editor);
                            }
                        });
                    });
                }
            });
        // FIXME: Issues with length not updating after element is removed. sometimes causes crash when closing windows
        for i in 0..self.sub_elements.len() {
            let mut is_open = true;
            println!("index: {}, length: {}", i, self.sub_elements.len());
            self.sub_elements[i].show(ctx, &mut is_open);
            if !is_open {
                self.sub_elements.remove(i);
            }
        }
    }
}
