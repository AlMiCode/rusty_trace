use std::cell::RefCell;
use std::rc::Rc;

use crate::camera::CameraSettings;
use crate::repo::{Id, Repo};
use crate::texture::Texture;
use crate::{render, scene::Scene};
use egui::color_picker::show_color;
use egui::{Color32, Ui};

use image::RgbImage;
use poll_promise::Promise;

use super::views;

pub trait GuiElement {
    fn show(&mut self, ctx: &egui::Context);
}

pub struct SceneEditor {
    title: String,
    scene: Rc<RefCell<Scene>>,

    texture_editor: (views::TextureEditor, bool),

    previews: Vec<(views::ImageView, bool)>,
}

impl SceneEditor {
    pub fn new(title: impl Into<String>, scene: Rc<RefCell<Scene>>) -> Self {
        Self {
            title: title.into(),
            scene: scene.clone(),
            texture_editor: (views::TextureEditor::new(scene), false),
            previews: Vec::new(),
        }
    }
}

impl GuiElement for SceneEditor {
    fn show(&mut self, ctx: &egui::Context) {
        let pos = egui::pos2(10.0, 10.0);

        show_view_as_window(
            ctx,
            &mut self.texture_editor.0,
            &mut self.texture_editor.1,
            true,
        );

        for (preview, open) in &mut self.previews {
            show_view_as_window(ctx, preview, open, false);
        }

        egui::Window::new(&self.title)
            .default_pos(pos)
            .vscroll(true)
            .show(ctx, |ui| {
                ui.heading("Scene");
                ui.group(|ui| {
                    if ui.link("Objects").clicked() {};
                    if ui.link("Materials").clicked() {};
                    if ui.link("Textures").clicked() {
                        self.texture_editor.1 = true;
                    }
                });
                ui.collapsing("Background", |ui| {
                    ui.horizontal(|ui| {
                        let mut background = self.scene.borrow().background;
                        texture_picker(ui, &mut background, &self.scene.borrow().textures.borrow());
                        self.scene.borrow_mut().background = background;
                    });
                });
                ui.collapsing("Objects", |ui| {
                    let hittable_len = self.scene.borrow().hittable.len();
                    for i in 0..hittable_len {
                        ui.collapsing(
                            format!("{} {i}", self.scene.borrow().hittable[i].name()),
                            |ui| {
                                ui.label("Position");
                                let mut c = self.scene.borrow().hittable[i].as_ref().get_position();
                                if views::point3_editor(ui, &mut c).changed() {
                                    let mut scene_ref_mut = self.scene.borrow_mut();
                                    scene_ref_mut.hittable[i].as_mut().set_position(c);
                                }
                            },
                        );
                    }
                });
                let cam_len = self.scene.borrow().cameras.len();
                for c in 0..cam_len {
                    ui.collapsing(format!("Camera {c}"), |ui| {
                        camera_settings_editor(ui, &mut self.scene.borrow_mut().cameras[c]);
                        ui.separator();
                        if ui.button("Render").clicked() {
                            let title = format!("Render {} for Camera {}", self.previews.len(), c);
                            let scene = (*self.scene.borrow()).clone();
                            let preview = views::ImageView::new(
                                title,
                                Promise::spawn_thread("debug-renderer", move || {
                                    let mut image = RgbImage::new(400, 400);
                                    render(
                                        &mut image,
                                        &scene.cameras[c].build_with_dimensions(400, 400),
                                        &scene.hittable,
                                        &scene.textures.borrow().get(scene.background),
                                        &scene.materials,
                                        &scene.textures.borrow(),
                                        &scene.images.borrow(),
                                        5,
                                        10,
                                    );
                                    image
                                }),
                            );
                            self.previews.push((preview, true));
                        }
                        ui.separator();
                    });
                }
            });
    }
}

fn texture_picker(ui: &mut Ui, tex_id: &mut Id<Texture>, repo: &Repo<Texture>) {
    egui::ComboBox::from_label("")
        .selected_text(format!("Texture {}", tex_id))
        .show_ui(ui, |ui| {
            ui.selectable_value(tex_id, Id::default(), "Default");
            for (option, _tex) in repo.iter() {
                ui.selectable_value::<Id<Texture>>(tex_id, *option, format!("Texture {}", option));
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
    egui::Grid::new(ui.auto_id_with("camera_settings")).show(ui, |ui| {
        ui.label("FOV:");
        ui.add(
            egui::DragValue::new(&mut c.fov)
                .speed(0.5)
                .clamp_range(0..=360)
                .suffix("°"),
        );
        ui.end_row();

        ui.label("Aperture:");
        ui.add(egui::DragValue::new(&mut c.aperture).speed(0.05));
        ui.end_row();

        ui.label("Look At:");
        views::point3_editor(ui, &mut c.look_at);
        ui.end_row();

        ui.label("Look From:");
        views::point3_editor(ui, &mut c.look_from);
        ui.end_row();
    });
}

fn show_view_as_window(
    ctx: &egui::Context,
    view: &mut dyn views::View,
    open: &mut bool,
    vscroll: bool,
) {
    egui::Window::new(view.title())
        .open(open)
        .vscroll(vscroll)
        .resizable(true)
        .show(ctx, |ui| view.ui(ui));
}
