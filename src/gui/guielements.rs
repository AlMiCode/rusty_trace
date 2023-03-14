use egui::ColorImage;
use egui_extras::RetainedImage;
use image::RgbImage;

pub trait GuiElement {
    fn show(&mut self, ctx: &egui::Context);
    fn get_thread_nr(&self) -> usize;
}

pub struct ImageGuiElement {
    thread_nr: usize,
    title: String,
    image: RetainedImage,
}

impl ImageGuiElement {
    pub fn new(thread_nr: usize, image: RgbImage) -> Self {
        let title = String::from("Render");
        Self {
            thread_nr,
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
        let pos = egui::pos2(16.0, 128.0 * (self.thread_nr as f32 + 1.0));
        egui::Window::new(&self.title)
            .default_pos(pos)
            .show(ctx, |ui| {
                self.image.show(ui);
            });
    }

    fn get_thread_nr(&self) -> usize {
        self.thread_nr
    }
}
