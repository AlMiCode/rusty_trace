use std::time::Instant;

use egui::{Color32, Separator};

use ray::oidn::OIND;
use ray::render::hittable::HittableVec;
use ray::render::material::Material;
use ray::render::render;
use ray::render::scene::{Scene, SceneRef};
use ray::render::texture::Texture;
use ray::vec_repo::{Id, VecRepo};

use super::logger::LOGGER;
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
    egui::SidePanel::new(side, egui::Id::new(view.title())).show_animated(ctx, open, |ui| {
        ui.heading(view.title());
        ui.separator();
        egui::ScrollArea::vertical().show(ui, |ui| view.ui(ui))
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
    previews: Vec<(views::RenderedImageView, bool)>,
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

    pub fn save_scene(&mut self) {
        rfd::FileDialog::new().save_file().and_then(|path| {
            let scene = SceneRef {
                hittable: &self.hittable,
                camera: self.cameras_editor.0.last_chosen_camera(),
                background: &self.background,
                materials: &self.materials,
                textures: self.texture_editor.0.get_repo(),
            };
            Some(
                rmp_serde::to_vec(&scene)
                    .map(|data| miniz_oxide::deflate::compress_to_vec(&data, 10))
                    .map(|contents| std::fs::write(path, contents))
                    .unwrap(),
            )
        });
    }

    pub fn load_scene(&mut self) {
        rfd::FileDialog::new()
            .pick_file()
            .and_then(|path| std::fs::read(path).ok())
            .and_then(|vec| miniz_oxide::inflate::decompress_to_vec(&vec).ok())
            .and_then(|vec| rmp_serde::from_slice(&vec).ok())
            .and_then(|scene: Scene| {
                self.cameras_editor = (
                    views::CamerasEditor::with_default(scene.camera),
                    self.cameras_editor.1,
                );
                self.texture_editor = (
                    views::TextureEditor::from(scene.textures),
                    self.texture_editor.1,
                );
                Some(42)
            });
    }
}

impl GuiElement for ProjectEditor {
    fn show(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel")
            .exact_height(30.0)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    if ui
                        .selectable_label(self.cameras_editor.1, "Cameras")
                        .clicked()
                    {
                        self.cameras_editor.1 = !self.cameras_editor.1;
                    }
                    if ui.selectable_label(false, "Objects").clicked() {}
                    if ui.selectable_label(false, "Materials").clicked() {}
                    if ui
                        .selectable_label(self.texture_editor.1, "Textures")
                        .clicked()
                    {
                        self.texture_editor.1 = !self.texture_editor.1;
                    }
                    if ui.selectable_label(false, "Load").clicked() {
                        self.load_scene()
                    }
                    if ui.selectable_label(false, "Save").clicked() {
                        self.save_scene()
                    }
                    ui.add_space(ui.available_width() - 240.0);
                    ui.label("Background:");
                    self.texture_editor
                        .0
                        .texture_picker(ui, &mut self.background);
                });
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .exact_height(18.0)
            .show(ctx, |ui| {
                ui.horizontal_top(|ui| {
                    let oidn_text =
                        format!("OIDN: {}", if OIND.availible() { "ON" } else { "OFF" });
                    ui.colored_label(
                        if OIND.availible() {
                            Color32::GREEN
                        } else {
                            Color32::RED
                        },
                        oidn_text,
                    );
                    ui.add(Separator::default().vertical());
                    LOGGER.with_latest(|str| {
                        ui.label(str);
                    });
                })
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
                textures: self.texture_editor.0.get_repo().clone(),
            };
            let (tx, rx) = std::sync::mpsc::channel();
            std::thread::spawn(move || {
                let now = Instant::now();
                tx.send(render((400, 400), &scene, 30, 30))
                    .expect("Successfully sent image");
                LOGGER.log(format!("Rendering finished in {:.2?}", now.elapsed()));
            });
            let preview = views::RenderedImageView::new(title, rx);
            self.previews.push((preview, true));
        }
    }
}
