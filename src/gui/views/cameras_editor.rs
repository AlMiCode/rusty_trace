use std::mem::take;

use crate::camera::CameraSettings;

use super::{point3_editor, View};

#[derive(Default)]
pub struct CamerasEditor {
    cameras: Vec<CameraSettings>,
    chosen_camera: Option<CameraSettings>,
}

impl CamerasEditor {
    pub fn chosen_camera(&mut self) -> Option<CameraSettings> {
        take(&mut self.chosen_camera)
    }

    pub fn add_camera(&mut self, c: CameraSettings) {
        self.cameras.push(c);
    }
}

impl View for CamerasEditor {
    fn title(&self) -> &str {
        "Cameras"
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("+").clicked() {
                self.cameras.push(Default::default());
            }
            if ui.button("-").clicked() {
                self.cameras.pop();
            }
        });
        for c in &mut self.cameras {
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
                point3_editor(ui, &mut c.look_at);
                ui.end_row();

                ui.label("Look From:");
                point3_editor(ui, &mut c.look_from);
                ui.end_row();
            });
            if ui.button("Render").clicked() {
                self.chosen_camera = Some(c.clone());
            }
        }
    }
}
