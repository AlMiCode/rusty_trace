use egui::{ColorImage, Ui, Response};

mod texture_editor;
use egui_extras::RetainedImage;
use image::RgbImage;
pub use texture_editor::TextureEditor;

mod image_view;
pub use image_view::ImageView;

use crate::Point3;

mod object_editor;

pub trait View {
    fn title(&self) -> &str;
    fn ui(&mut self, ui: &mut Ui);
}

/// Helper struct. Not for public use
struct Image<'a>(&'a RgbImage);
impl Into<RetainedImage> for Image<'_> {
    fn into(self) -> RetainedImage {
        RetainedImage::from_color_image(
            "opened file",
            ColorImage::from_rgb(
                [self.0.width() as usize, self.0.height() as usize],
                self.0.as_raw(),
            ),
        )
    }
}

pub fn point3_editor(ui: &mut Ui, p: &mut Point3) -> Response {
    ui.horizontal(|ui| {
        let x_field = ui.add(egui::DragValue::new(&mut p.x).speed(0.05).prefix("X: "));
        let y_field = ui.add(egui::DragValue::new(&mut p.y).speed(0.05).prefix("Y: "));
        let z_field = ui.add(egui::DragValue::new(&mut p.z).speed(0.05).prefix("Z: "));
        x_field.union(y_field).union(z_field)
    })
    .inner
}
