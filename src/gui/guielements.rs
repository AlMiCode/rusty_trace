use image::RgbImage;
use poll_promise::Promise;

use crate::hittable::HittableVec;
use crate::material::Material;
use crate::render;
use crate::repo::{Id, VecRepo};
use crate::scene::Scene;
use crate::texture::Texture;

use super::views;

pub trait GuiElement {
    fn show(&mut self, ctx: &egui::Context);
}

fn show_view_as_window(
    ctx: &egui::Context,
    view: &mut impl views::View,
    open: &mut bool,
    vscroll: bool,
) {
    egui::Window::new(view.title())
        .open(open)
        .vscroll(vscroll)
        .resizable(true)
        .show(ctx, |ui| view.ui(ui));
}

fn show_view_as_side_panel(
    ctx: &egui::Context,
    view: &mut impl views::View,
    open: bool,
    side: egui::panel::Side,
) {
    egui::SidePanel::new(side, egui::Id::new(view.title()))
        .show_animated(ctx, open, |ui| {
            ui.heading(view.title());
            ui.separator();
            view.ui(ui)
        });
}

#[derive(Default)]
pub struct ProjectEditor {
    //temporary. will be replaced with editors later.
    hittable: HittableVec,
    background: Id<Texture>,
    materials: VecRepo<Material>,
    //===========================
    cameras_editor: (views::CamerasEditor, bool),
    texture_editor: (views::TextureEditor, bool),
    previews: Vec<(views::ImageView, bool)>,
}

impl ProjectEditor {
    pub fn from_scene(scene: Scene) -> Self {
        let cameras_editor = (views::CamerasEditor::with_default(scene.camera), false);
        let texture_editor = (views::TextureEditor::from(scene.textures), false);

        Self {
            hittable: scene.hittable,
            background: scene.background,
            materials: scene.materials,
            cameras_editor,
            texture_editor,
            previews: Vec::new(),
        }
    }
}

impl GuiElement for ProjectEditor {
    fn show(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").exact_height(30.0).show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                if ui.selectable_label(self.cameras_editor.1, "Cameras").clicked() {
                    self.cameras_editor.1 = !self.cameras_editor.1;
                }
                if ui.selectable_label(false, "Objects").clicked() {}
                if ui.selectable_label(false, "Materials").clicked() {}
                if ui.selectable_label(self.texture_editor.1, "Textures").clicked() {
                    self.texture_editor.1 = !self.texture_editor.1;
                }
                ui.add_space(ui.available_width() - 240.0);
                ui.label("Background:");
                self.texture_editor.0.texture_picker(ui, &mut self.background);
            });
        });


        show_view_as_side_panel(
            ctx,
            &mut self.cameras_editor.0,
            self.cameras_editor.1,
            egui::panel::Side::Left,
        );

        show_view_as_side_panel(
            ctx,
            &mut self.texture_editor.0,
            self.texture_editor.1,
            egui::panel::Side::Right,
        );

        for (preview, open) in &mut self.previews {
            show_view_as_window(ctx, preview, open, false);
        }

        if let Some(camera) = self.cameras_editor.0.chosen_camera() {
            let title = format!("Render {}", self.previews.len());
            let scene = Scene {
                hittable: self.hittable.clone(),
                camera,
                background: self.background,
                materials: self.materials.clone(),
                textures: self.texture_editor.0.get_repo(),
            };
            let preview = views::ImageView::new(
                title,
                Promise::spawn_thread("debug-renderer", move || {
                    let mut image = RgbImage::new(400, 400);
                    render(&mut image, &scene, 1, 10);
                    image
                }),
            );
            self.previews.push((preview, true));
        }
    }
}
