use std::cell::RefCell;
use std::rc::Rc;

use crate::Point3;
use crate::camera::CameraSettings;
use crate::repo::{Id, Repo};
use crate::texture::Texture;
use crate::{render, scene::Scene};
use egui::color_picker::show_color;
use egui::{Color32, Response, Ui, ColorImage};
use egui_extras::{RetainedImage};
use image::RgbImage;
use poll_promise::Promise;

use super::views::{self, View};

pub trait GuiElement {
    fn show(&mut self, ctx: &egui::Context);
}

pub struct ImageGuiElement {
    title: String,
    image: Promise<RetainedImage>,
    dimensions: (u32, u32),
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
                    &scene.cameras[camera_id].build_with_dimensions(img_dimensions.0, img_dimensions.1),
                    &scene.hittable,
                    &scene.textures.borrow().get(scene.background),
                    &scene.materials,
                    &scene.textures.borrow(),
                    &scene.images.borrow(),
                    5,
                    10,
                );
                RetainedImage::from_color_image(
                    "render",
                    ColorImage::from_rgb(
                        [image.width() as usize, image.height() as usize],
                        image.as_raw(),
                    ),
                )
            }),
            dimensions: img_dimensions,
            is_open: true,
        }
    }
}

impl GuiElement for ImageGuiElement {
    fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new(&self.title)
            .open(&mut self.is_open)
            .min_width(self.dimensions.0 as f32)
            .min_height(self.dimensions.1 as f32)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| match self.image.ready() {
                    None => {
                        ui.spinner();
                        ui.label("Rendering in progress...")
                    }
                    Some(image) => image.show(ui),
                });
            });
    }
}

pub struct SceneEditor {
    title: String,
    scene: Rc<RefCell<Scene>>,
    sub_elements: Vec<Box<dyn GuiElement>>,
    texture_editor: TextureEditor,
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
        }
    }
}

impl GuiElement for SceneEditor {
    fn show(&mut self, ctx: &egui::Context) {
        let pos = egui::pos2(10.0, 10.0);

        self.texture_editor.show(ctx);

        egui::Window::new(&self.title)
            .default_pos(pos)
            .vscroll(true)
            .show(ctx, |ui| {
                ui.heading("Scene");
                ui.group(|ui| {
                    if ui.link("Objects").clicked() {};
                    if ui.link("Materials").clicked() {};
                    if ui.link("Textures").clicked() {
                        self.texture_editor.is_open = true;
                    }
                });
                ui.collapsing("Background", |ui| {
                    ui.horizontal(|ui| {
                        let mut background = self.scene.borrow().background;
                        texture_picker(
                            ui,
                            &mut background,
                            &self.scene.borrow().textures.borrow(),
                        );
                        self.scene.borrow_mut().background = background;
                    });
                });
                ui.collapsing("Objects", |ui| {
                    let hittable_len = self.scene.borrow().hittable.len();
                    for i in 0..hittable_len {
                        ui.collapsing(format!("{} {i}", self.scene.borrow().hittable[i].name()), |ui| {
                            ui.label("Position");
                            let mut c = self.scene.borrow().hittable[i].as_ref().get_position();
                            if point3_editor(ui, &mut c).changed() {
                                let mut scene_ref_mut = self.scene.borrow_mut();
                                scene_ref_mut.hittable[i].as_mut().set_position(c);
                            }
                        });
                    }
                });
                let cam_len = self.scene.borrow().cameras.len();
                for c in 0..cam_len {
                    ui.collapsing(format!("Camera {c}"), |ui| {
                        camera_settings_editor(ui, &mut self.scene.borrow_mut().cameras[c]);
                        ui.separator();
                        if ui.button("Render").clicked() {
                            let guielement = Box::new(ImageGuiElement::new(
                                c,
                                self.sub_elements.len(),
                                (400, 400),
                                (*self.scene.borrow()).clone(),
                            ));
                            self.sub_elements.push(guielement);
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

fn point3_editor(ui: &mut Ui, p: &mut Point3) -> Response {
    ui.horizontal(|ui| {
        let x_field = ui.add(egui::DragValue::new(&mut p.x).speed(0.05).prefix("X: "));
        let y_field = ui.add(egui::DragValue::new(&mut p.y).speed(0.05).prefix("Y: "));
        let z_field = ui.add(egui::DragValue::new(&mut p.z).speed(0.05).prefix("Z: "));
        x_field.union(y_field).union(z_field)
    })
    .inner
}

fn texture_picker(ui: &mut Ui, tex_id: &mut Id<Texture>, repo: &Repo<Texture>) {
    egui::ComboBox::from_label("")
        .selected_text(format!("Texture {}", tex_id))
        .show_ui(ui, |ui| {
            ui.selectable_value(tex_id, Id::default(), "Default");
            for (option, _tex) in repo.iter() {
                ui.selectable_value::<Id<Texture>>(
                    tex_id,
                    *option,
                    format!("Texture {}", option),
                );
            }
        });
    if let Texture::Colour(c) = repo.get(*tex_id) {
        let colour: Color32 = egui::Rgba::from_rgb(c.x, c.y, c.z).into();
        show_color(ui, colour, egui::vec2(35.0, 15.0));
    } else {
        ui.label("Image");
    }
}

fn camera_settings_editor(ui: &mut Ui, c: &mut CameraSettings) {
    egui::Grid::new(ui.auto_id_with("camera_settings")).show(ui, |ui|{
        ui.label("FOV:");
        ui.add(egui::DragValue::new(&mut c.fov).speed(0.5).clamp_range(0..=360).suffix("°"));
        ui.end_row();

        ui.label("Aperture:");
        ui.add(egui::DragValue::new(&mut c.aperture).speed(0.05));
        ui.end_row();

        ui.label("Look At:");
        point3_editor(ui, &mut c.look_at);
        ui.end_row();

        ui.label("Look From:");
        point3_editor(ui, &mut c.look_from);
        ui.end_row();
    });
}

pub fn load_image(filename: String) -> Option<RgbImage> {
    let reader = match image::io::Reader::open(&filename) {
        Err(_) => {
            eprintln!("Could not read");
            return None;
        }
        Ok(r) => r,
    };
    let dyn_img = match reader.decode() {
        Err(_) => {
            eprintln!("Could not decode");
            return None;
        }
        Ok(img) => img,
    };
    Some(dyn_img.to_rgb8())
}

struct TextureEditor {
    view: views::TextureEditor,
    pub is_open: bool,
}

impl TextureEditor {
    fn new(scene: Rc<RefCell<Scene>>) -> Self {
        TextureEditor {
            view: views::TextureEditor::new(scene),
            is_open: true,
        }
    }
}

impl GuiElement for TextureEditor {
    fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("Textures")
            .open(&mut self.is_open)
            .vscroll(true)
            .show(ctx, |ui| self.view.ui(ui));
    }
}
