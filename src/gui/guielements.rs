use std::cell::RefCell;
use std::rc::Rc;

use crate::repo::Id;
use crate::texture::Texture;
use crate::{render, scene::Scene};
use egui::color_picker::{color_edit_button_rgb, show_color};
use egui::Color32;
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
    is_open: bool,
}

impl ImageGuiElement {
    pub fn new(
        camera_id: usize,
        render_id: usize,
        img_dimensions: (u32, u32),
        scene: Scene,
    ) -> Self {
        let title = format!("Render {render_id} for camera {camera_id}");
        Self {
            title,
            image: Promise::spawn_thread("debug-renderer", move || {
                let mut image = RgbImage::new(img_dimensions.0, img_dimensions.1);
                render(
                    &mut image,
                    &scene.cameras[camera_id],
                    &scene.hittable,
                    &scene.textures.get(scene.background),
                    &scene.materials,
                    &scene.textures,
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
            is_open: true,
        }
    }
}

impl GuiElement for ImageGuiElement {
    fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new(&self.title)
            .open(&mut self.is_open)
            .show(ctx, |ui| match self.image.ready() {
                None => ui.spinner(),
                Some(image) => image.show(ui),
            });
    }
}

pub struct SceneEditor {
    title: String,
    scene: Rc<RefCell<Scene>>,
    sub_elements: Vec<Box<dyn GuiElement>>,
    texture_editor: TextureEditor,
    is_open: bool,
}

impl SceneEditor {
    pub fn new(title: impl Into<String>, scene: Rc<RefCell<Scene>>) -> Self {
        let mut texture_editor = TextureEditor::new(scene.clone());
        texture_editor.is_open = false;
        Self {
            title: title.into(),
            scene,
            sub_elements: vec![],
            texture_editor,
            is_open: true,
        }
    }
}

impl GuiElement for SceneEditor {
    fn show(&mut self, ctx: &egui::Context) {
        let pos = egui::pos2(10.0, 10.0);

        self.texture_editor.show(ctx);

        egui::Window::new(&self.title)
            .default_pos(pos)
            .open(&mut self.is_open)
            .vscroll(true)
            .show(ctx, |ui| {
                ui.group(|ui|{
                    if ui.link("Objects").clicked() {};
                    if ui.link("Materials").clicked() {};
                    if ui.link("Textutes").clicked() {
                        self.texture_editor.is_open = true;
                    }
                });
                ui.collapsing("Scene", |ui| {
                    ui.collapsing("Background", |ui| {
                        ui.horizontal(|ui| {
                            let tex = self.scene.borrow().background;
                            let mut new_background = tex;
                            egui::ComboBox::from_label("")
                            .selected_text(format!("Texture {}", tex))
                            .show_ui(ui, |ui|{
                                ui.selectable_value(&mut new_background, Id::default(), "Default");
                                for (option, _tex) in self.scene.borrow().textures.iter() {
                                    ui.selectable_value(&mut new_background, *option, format!("Texture {}", option));
                                }
                            });
                            if let Texture::Colour(c) = self.scene.borrow().textures.get(tex) {
                                let colour: Color32 = egui::Rgba::from_rgb(c.x, c.y, c.z).into();
                                show_color(ui, colour, egui::vec2(35.0, 15.0));
                            } else {
                                ui.label("Image. Unimplemented");
                            }
                            self.scene.borrow_mut().background = new_background;
                        });
                    });
                    ui.collapsing("Objects", |ui| {
                        let hittable_len = self.scene.borrow().hittable.len();
                        for i in 0..hittable_len {
                            ui.collapsing(format!("Sphere {i}"), |ui| {
                                ui.label("Position");
                                ui.horizontal(|ui| {
                                    let mut c = self.scene.borrow().hittable[i].as_ref().get_position();
                                    ui.label("X: ");
                                    let x = ui.add(egui::DragValue::new(&mut c.x)).changed();
                                    ui.label("Y: ");
                                    let y = ui.add(egui::DragValue::new(&mut c.y)).changed();
                                    ui.label("z: ");
                                    let z = ui.add(egui::DragValue::new(&mut c.z)).changed();
                                    if x || y || z {
                                        let mut scene_ref_mut = self.scene.borrow_mut();
                                        scene_ref_mut.hittable[i].as_mut().set_position(c);
                                    }
                                })
                            });
                        }
                    })
                });
                let cam_len = self.scene.borrow().cameras.len();
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
                                        (*self.scene.borrow()).clone(),
                                    ));
                                    self.sub_elements.push(guielement);
                                }
                            });
                        }
                        // fov dragvalue
                        {
                            ui.horizontal(|ui| {
                                let mut fov: f64 = self.scene.borrow().cameras[c].settings.fov;
                                ui.label("Fov:");
                                if ui.add(egui::DragValue::new(&mut fov)).changed() {
                                    let mut scene_ref_mut = self.scene.borrow_mut();
                                    scene_ref_mut.cameras[c].settings.fov = fov;
                                    scene_ref_mut.cameras[c].update();
                                }
                            });
                        }
                        // aperture dragvalue
                        {
                            ui.horizontal(|ui| {
                                let mut aperture: f64 = self.scene.borrow().cameras[c].settings.aperture;
                                ui.label("Aperture:");
                                if ui.add(egui::DragValue::new(&mut aperture)).changed() {
                                    let mut scene_ref_mut = self.scene.borrow_mut();
                                    scene_ref_mut.cameras[c].settings.aperture = aperture;
                                    scene_ref_mut.cameras[c].update();
                                }
                            });
                        }
                        // look_at dragvalues
                        {
                            ui.collapsing("Look at", |ui| {
                                ui.horizontal(|ui| {
                                    let mut x: f64 = self.scene.borrow().cameras[c].settings.look_at.x;
                                    ui.label("x:");
                                    if ui.add(egui::DragValue::new(&mut x)).changed() {
                                        let mut scene_ref_mut = self.scene.borrow_mut();
                                        scene_ref_mut.cameras[c].settings.look_at.x = x;
                                        scene_ref_mut.cameras[c].update();
                                    }
                                    let mut y: f64 = self.scene.borrow().cameras[c].settings.look_at.y;
                                    ui.label("y:");
                                    if ui.add(egui::DragValue::new(&mut y)).changed() {
                                        let mut scene_ref_mut = self.scene.borrow_mut();
                                        scene_ref_mut.cameras[c].settings.look_at.y = y;
                                        scene_ref_mut.cameras[c].update();
                                    }
                                    let mut z: f64 = self.scene.borrow().cameras[c].settings.look_at.z;
                                    ui.label("z:");
                                    if ui.add(egui::DragValue::new(&mut z)).changed() {
                                        let mut scene_ref_mut = self.scene.borrow_mut();
                                        scene_ref_mut.cameras[c].settings.look_at.z = z;
                                        scene_ref_mut.cameras[c].update();
                                    }
                                })
                            });
                        }
                        {
                            ui.collapsing("Look from", |ui| {
                                ui.horizontal(|ui| {
                                    let mut x: f64 = self.scene.borrow().cameras[c].settings.look_from.x;
                                    ui.label("x:");
                                    if ui.add(egui::DragValue::new(&mut x)).changed() {
                                        let mut scene_ref_mut = self.scene.borrow_mut();
                                        scene_ref_mut.cameras[c].settings.look_from.x = x;
                                        scene_ref_mut.cameras[c].update();
                                    }
                                    let mut y: f64 = self.scene.borrow().cameras[c].settings.look_from.y;
                                    ui.label("y:");
                                    if ui.add(egui::DragValue::new(&mut y)).changed() {
                                        let mut scene_ref_mut = self.scene.borrow_mut();
                                        scene_ref_mut.cameras[c].settings.look_from.y = y;
                                        scene_ref_mut.cameras[c].update();
                                    }
                                    let mut z: f64 = self.scene.borrow().cameras[c].settings.look_from.z;
                                    ui.label("z:");
                                    if ui.add(egui::DragValue::new(&mut z)).changed() {
                                        let mut scene_ref_mut = self.scene.borrow_mut();
                                        scene_ref_mut.cameras[c].settings.look_from.z = z;
                                        scene_ref_mut.cameras[c].update();
                                    }
                                })
                            });
                        }
                        ui.separator();
                    });
                }
            });
        // FIXME: Issues with length not updating after element is removed. sometimes causes crash when closing windows
        //self.sub_elements = self.sub_elements.drain(..).filter(|(_e, is_open)| *is_open).collect();
        for e in &mut self.sub_elements {
            e.show(ctx);
        }
    }
}



struct TextureEditor {
    scene: Rc<RefCell<Scene>>,
    edited_id: Option<Id<Texture>>,
    rgb: [f32; 3],
    //image: RgbImage,
    choosing_colour: bool,
    pub is_open: bool,
}

impl TextureEditor {
    fn new(scene: Rc<RefCell<Scene>>) -> Self {
        TextureEditor {
            scene,
            edited_id: None,
            rgb: [0f32, 0f32, 0f32],
            //image: RgbImage::new(4, 4),
            choosing_colour: true,
            is_open: true,
        }
    }
}

impl GuiElement for TextureEditor {
    fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("Textures Editor")
            .open(&mut self.is_open)
            .show(ctx, |ui| {
                ui.group(|ui| {
                    ui.label("Default");
                    ui.horizontal(|ui| {
                        if let Texture::Colour(c) = self.scene.borrow().textures.get_default() {
                            ui.label("Colour: ");
                            let colour: Color32 = egui::Rgba::from_rgb(c.x, c.y, c.z).into();
                            show_color(ui, colour, egui::vec2(35.0, 15.0));
                        } else {
                            ui.label("Image: Unimplemented");
                        }
                    });
                });

                let mut scene_ref_mut = self.scene.borrow_mut();
                for (id, tex) in scene_ref_mut.textures.iter_mut() {
                    ui.group(|ui| {
                        ui.label(format!("Texture {}", id));
                        ui.horizontal(|ui| {
                            if let Texture::Colour(c) = tex.as_ref() {
                                ui.label("Colour: ");
                                let colour: Color32 = egui::Rgba::from_rgb(c.x, c.y, c.z).into();
                                show_color(ui, colour, egui::vec2(35.0, 15.0));
                            } else {
                                ui.label("Image: Unimplemented");
                            }
                            if ui.button("Change").clicked() {
                                self.edited_id = Some(*id);
                                if let Texture::Colour(c) = tex.as_ref() {
                                    self.choosing_colour = true;
                                    self.rgb = [c.x, c.y, c.z];
                                }
                            }
                        });
                        if let Some(edited_id) = self.edited_id {
                            if edited_id == *id {
                                ui.horizontal(|ui| {
                                    ui.selectable_value(&mut self.choosing_colour, true, "Colour");
                                    ui.selectable_value(&mut self.choosing_colour, false, "Image");
                                });
                                ui.horizontal(|ui| {
                                    if self.choosing_colour {
                                        color_edit_button_rgb(ui, &mut self.rgb);
                                        if ui.button("Set").clicked() {
                                            *tex.as_mut() = Texture::Colour(self.rgb.into());
                                        }
                                    } else {
                                        ui.label("File picker. Unimplemented");
                                    }
                                    if ui.button("Cancel").clicked() {
                                        self.edited_id = None;
                                    }
                                });
                            }
                        }
                    });
                }
            });
    }
}
