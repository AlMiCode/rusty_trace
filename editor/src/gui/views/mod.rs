use egui::{Response, Ui};

mod texture_editor;

pub use texture_editor::TextureEditor;

mod image_view;
pub use image_view::RenderedImageView;

mod cameras_editor;
pub use cameras_editor::CamerasEditor;

use ray::render::Point3;

mod object_editor;

pub trait View {
    fn title(&self) -> &str;
    fn ui(&mut self, ui: &mut Ui);
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

fn grid(ui: &mut Ui, name: &'static str, num_columns: usize, stripped: bool) -> egui::Grid {
    egui::Grid::new(ui.auto_id_with(name))
        .num_columns(num_columns)
        .striped(stripped)
}
