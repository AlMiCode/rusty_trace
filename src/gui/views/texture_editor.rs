use std::{rc::Rc, cell::RefCell, sync::Arc};

use egui::{ColorImage, Ui, Color32, color_picker::show_color, RichText, Vec2};
use egui_extras::RetainedImage;
use image::RgbImage;
use indexmap::IndexMap;
use poll_promise::Promise;

use crate::{scene::Scene, texture::Texture, repo::Id, gui::guielements::load_image};

use super::View;

pub struct TextureEditor {
    scene: Rc<RefCell<Scene>>,
    // Editor
    edited_id: Option<Id<Texture>>,
    edited_rgb: [f32; 3],
    edited_image_id: Option<Id<RgbImage>>,
    loaded_image: Promise<Option<Arc<RgbImage>>>,
    choosing_colour: bool,

    //RetaindedImage storage. Possibly moved into its own window.
    images: RefCell<IndexMap<Id<RgbImage>, RetainedImage>>,
    fallback: RetainedImage,
}

impl TextureEditor {
    pub fn new(scene: Rc<RefCell<Scene>>) -> Self {
        TextureEditor {
            scene,
            edited_id: None,
            edited_rgb: [0f32, 0f32, 0f32],
            edited_image_id: None,
            loaded_image: Promise::from_ready(None),
            choosing_colour: true,
            images: RefCell::new(IndexMap::new()),
            fallback: RetainedImage::from_color_image("Fallback", ColorImage::example()),
        }
    }
}

impl View for TextureEditor {
    fn ui(&mut self, ui: &mut Ui) {
        egui::Grid::new(ui.auto_id_with("textures"))
            .num_columns(4)
            .striped(true)
            .show(ui, |ui| {
            for (id, tex) in self.scene.borrow().textures.borrow_mut().iter_mut() {
                ui.label(format!("Texture {}", id));
                if let Texture::Colour(c) = tex.as_ref() {
                    ui.label("Colour");
                    let colour: Color32 = egui::Rgba::from_rgb(c.x, c.y, c.z).into();
                    show_color(ui, colour, egui::vec2(35.0, 15.0));
                } else if let Texture::Image(img) = tex.as_ref() {
                    ui.label("Image");
                    ui.label(RichText::new("example.png").italics().underline()).on_hover_ui(|ui| {
                        let images_ref = self.images.borrow();
                        let ret_img = images_ref.get(img).unwrap_or(&self.fallback);
                        ret_img.show_size(ui, Vec2::new(100f32, 100f32));
                    });
                }
                if self.edited_id == None {
                    if ui.button("Edit").clicked() {
                        self.edited_id = Some(*id);
                        if let Texture::Colour(c) = tex.as_ref() {
                            self.choosing_colour = true;
                            self.edited_rgb = [c.x, c.y, c.z];
                        } else if let Texture::Image(img) = tex.as_ref() {
                            self.choosing_colour = false;
                            self.edited_image_id = Some(*img);
                        }
                    }
                } else {
                    let edited_id = self.edited_id.unwrap();
                    if edited_id == *id {
                        ui.end_row();
                        ui.label("");
                        ui.vertical(|ui| {
                            ui.style_mut().wrap = Some(false);
                            ui.radio_value(&mut self.choosing_colour, true, "Colour");
                            ui.radio_value(&mut self.choosing_colour, false, "Image");
                        });
                        if self.choosing_colour {
                            ui.color_edit_button_rgb(&mut self.edited_rgb);
                        } else {
                            if self.edited_image_id == None {
                                match self.loaded_image.poll() {
                                    std::task::Poll::Ready(None) => if ui.button("Open file...").clicked() {
                                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                                            let picked_path = path.display().to_string();
                                            self.loaded_image = load_image(picked_path);
                                            self.edited_image_id = None;
                                        }
                                    },
                                    std::task::Poll::Pending => {
                                        ui.spinner();
                                    }
                                    std::task::Poll::Ready(Some(img)) => {
                                        let img_id = self.scene.borrow().add_image(img.clone());
                                        self.images.borrow_mut().insert(img_id, RetainedImage::from_color_image(
                                                "opened file",
                                                ColorImage::from_rgb([img.width() as usize, img.height() as usize], img.as_raw())
                                            ),
                                        );
                    
                                        self.edited_image_id = Some(img_id);
                                        self.loaded_image = Promise::from_ready(None);
                                    }
                                }
                            } else {
                                let images_ref = self.images.borrow();
                                let ret_img = images_ref
                                    .get(&self.edited_image_id.unwrap())
                                    .unwrap_or(&self.fallback);
                                ret_img.show_size(ui, Vec2::new(100f32, 100f32));
                            }
                        }
                        ui.vertical(|ui|{
                            ui.style_mut().wrap = Some(false);
                            if ui.button("Save").clicked() {
                                if self.choosing_colour {
                                    *tex.as_mut() = Texture::Colour(self.edited_rgb.into());
                                } else {
                                    if let Some(img_id) = self.edited_image_id {
                                        *tex.as_mut() = Texture::Image(img_id);
                                    }
                                }
                                self.edited_id = None;
                                self.edited_image_id = None;
                                self.loaded_image = Promise::from_ready(None);
                            }
                            if ui.button("Cancel").clicked() {
                                self.edited_id = None;
                                self.edited_image_id = None;
                                self.loaded_image = Promise::from_ready(None);
                            }
                        });
                    }
                }
                ui.end_row();
            }
        });
    }
}
