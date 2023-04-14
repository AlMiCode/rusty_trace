use egui::{Ui, ColorImage};
use egui_extras::RetainedImage;
use image::RgbImage;
use poll_promise::Promise;

use crate::{scene::Scene, render};

use super::View;

pub struct Preview {
    image: Promise<RetainedImage>,
}

impl Preview {
    pub fn new(
        camera_id: usize,
        render_id: usize,
        img_dimensions: (u32, u32),
        scene: Scene,
    ) -> Self {
        let title = format!("Render {render_id} for camera {camera_id}");
        Self {
            image: Promise::spawn_thread("debug-renderer", move || {
                let mut image = RgbImage::new(img_dimensions.0, img_dimensions.1);
                render(
                    &mut image,
                    &scene.cameras[camera_id].build_with_dimensions(img_dimensions.0, img_dimensions.1),
                    &scene.hittable,
                    &scene.textures.borrow().get(scene.background),
                    &scene.materials,
                    &scene.textures.borrow(),
                    &scene.images.borrow(),
                    5,
                    10,
                );
                RetainedImage::from_color_image(
                    "render",
                    ColorImage::from_rgb(
                        [image.width() as usize, image.height() as usize],
                        image.as_raw(),
                    ),
                )
            }),
        }
    }
}

impl View for Preview {
    fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| match self.image.ready() {
            None => {
                ui.spinner();
                ui.label("Rendering in progress...")
            }
            Some(image) => image.show(ui),
        });
    }
}