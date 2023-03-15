use egui::ColorImage;
use egui_extras::RetainedImage;
use image::RgbImage;

pub trait GuiElement {
    fn show(&mut self, ctx: &egui::Context);
}

pub struct ImageGuiElement {
    title: String,
    image: RetainedImage,
}

impl ImageGuiElement {
    pub fn new(window_id: usize, image: RgbImage) -> Self {
        let title = format!("Render {window_id}");
        Self {
            title,
            image: RetainedImage::from_color_image(
                "render",
                ColorImage::from_rgb(
                    [image.width() as usize, image.height() as usize],
                    image.as_raw(),
                ),
            ),
        }
    }
}

impl GuiElement for ImageGuiElement {
    fn show(&mut self, ctx: &egui::Context) {
        let pos = egui::pos2(16.0, 128.0);
        egui::Window::new(&self.title)
            .default_pos(pos)
            .show(ctx, |ui| {
                self.image.show(ui);
            });
    }
}
