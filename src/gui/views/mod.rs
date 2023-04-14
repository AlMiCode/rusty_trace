use egui::{Ui, ColorImage};

mod texture_editor;
use egui_extras::RetainedImage;
use image::RgbImage;
pub use texture_editor::TextureEditor;

mod image_view;
pub use image_view::ImageView;

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