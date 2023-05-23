use std::mem::take;

use egui::RichText;

use ray::render::camera::CameraSettings;

use super::{point3_editor, View};

#[derive(Default)]
pub struct CamerasEditor {
    cameras: Vec<CameraSettings>,
    chosen_camera: Option<CameraSettings>,
    last_chosen_camera: CameraSettings,
    default: CameraSettings,
}

impl CamerasEditor {
    pub fn with_default(c: CameraSettings) -> Self {
        Self {
            cameras: vec![],
            chosen_camera: None,
            last_chosen_camera: c.clone(),
            default: c,
        }
    }

    pub fn chosen_camera(&mut self) -> Option<CameraSettings> {
        if let Some(ref chosen_camera) = self.chosen_camera {
            self.last_chosen_camera = chosen_camera.clone();
        }
        take(&mut self.chosen_camera)
    }

    pub fn last_chosen_camera(&self) -> &CameraSettings {
        &self.last_chosen_camera
    }
}

impl View for CamerasEditor {
    fn title(&self) -> &str {
        "Cameras"
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("+").clicked() {
                self.cameras.push(self.default.clone());
            }
            if ui.button("-").clicked() {
                self.cameras.pop();
            }
            if ui.button("Reset Default").clicked() {
                self.default = Default::default();
            }
        });
        ui.separator();

        egui::Grid::new(ui.auto_id_with("default_camera_settings")).show(ui, |ui| {
            ui.label(RichText::new("Default Camera").strong());
            ui.horizontal(|ui| {
                if ui.button("Render").clicked() {
                    self.chosen_camera = Some(self.default.clone());
                }
            });
            ui.end_row();
            ui.label("FOV:");
            ui.add(
                egui::DragValue::new(&mut self.default.fov)
                    .speed(0.5)
                    .clamp_range(0..=360)
                    .suffix("°"),
            );
            ui.end_row();

            ui.label("Aperture:");
            ui.add(egui::DragValue::new(&mut self.default.aperture).speed(0.05));
            ui.end_row();

            ui.label("Look At:");
            point3_editor(ui, &mut self.default.look_at);
            ui.end_row();

            ui.label("Look From:");
            point3_editor(ui, &mut self.default.look_from);
            ui.end_row();
        });

        ui.separator();

        for (idx, c) in &mut self.cameras.iter_mut().enumerate() {
            egui::Grid::new(ui.auto_id_with("camera_settings")).show(ui, |ui| {
                ui.label(RichText::new(format!("Camera {}", idx)).strong());
                ui.horizontal(|ui| {
                    if ui.button("Render").clicked() {
                        self.chosen_camera = Some(c.clone());
                    }
                    if ui.button("Set as Default").clicked() {
                        self.default = c.clone();
                    }
                });
                ui.end_row();
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
                point3_editor(ui, &mut c.look_at);
                ui.end_row();

                ui.label("Look From:");
                point3_editor(ui, &mut c.look_from);
                ui.end_row();
            });
        }
    }
}
