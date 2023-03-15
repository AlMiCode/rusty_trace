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
    pub fn new(image: RgbImage) -> Self {
        let title = String::from("Render");
        Self {
            title,
            image: RetainedImage::from_color_image(
                "render",
                ColorImage::from_rgb(
                    [image.dimensions().0 as usize, image.dimensions().1 as usize],
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
