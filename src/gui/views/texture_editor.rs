use std::{cell::RefCell, mem::take, rc::Rc, sync::Arc};

use egui::{color_picker::show_color, Color32, ColorImage, RichText, Ui, Vec2, Rect};
use egui_extras::RetainedImage;
use image::RgbImage;
use indexmap::IndexMap;

use crate::{io::load_image, repo::Id, scene::Scene, texture::Texture, Colour};

use super::{Image, View};

#[derive(Default)]
struct TextureEditorState {
    edited_id: Option<Id<Texture>>,
    choosing_image: bool,
    edited_rgb: [f32; 3],
    edited_image_id: Option<Id<RgbImage>>,
}

impl TextureEditorState {
    fn setup(&mut self, id: Id<Texture>, tex: &Texture) {
        self.edited_id = Some(id);
        match tex {
            Texture::Colour(c) => {
                self.choosing_image = false;
                self.edited_rgb = [c.x, c.y, c.z];
                self.edited_image_id = None;
            }
            Texture::Image(img) => {
                self.choosing_image = true;
                self.edited_image_id = Some(*img);
            }
        }
    }
}

#[derive(Default)]
pub struct TextureEditor {
    editor_state: TextureEditorState,
    textures: IndexMap<Id<Texture>, Texture>,
    images: IndexMap<Id<RgbImage>, Arc<RgbImage>>,
    retained_images: IndexMap<Id<RgbImage>, RetainedImage>,
}

impl TextureEditor {
    pub fn mock() -> Self {
        let mut textures = IndexMap::<Id<Texture>, Texture>::default();
        textures.insert(Id::new(), Colour::new(0.5, 0.5, 0.5).into());
        textures.insert(Id::new(), Colour::new(0.5, 0.5, 0.5).into());
        textures.insert(Id::new(), Colour::new(0.5, 0.5, 0.5).into());
        textures.insert(Id::new(), Colour::new(0.5, 0.5, 0.5).into());
        textures.insert(Id::new(), Colour::new(0.5, 0.5, 0.5).into());
        Self { textures, ..Default::default()}
    }

    fn add_image(&mut self, image: RgbImage) -> Id<RgbImage> {
        let id = Id::new();
        self.retained_images.insert(id, Image(&image).into());
        self.images.insert(id, Arc::new(image));
        id
    }

    fn image_preview(&self, ui: &mut Ui, id: &Id<RgbImage>, max_size: f32) {
        let fallback: RetainedImage =
            RetainedImage::from_color_image("Fallback", ColorImage::example());

        let image = self.retained_images
            .get(id)
            .unwrap_or(&fallback);

        let [width, height] = image.size();
        let (width, height) = (width as f32, height as f32);
        if width >= height {
            image.show_scaled(ui, max_size / width);
        } else {
            image.show_scaled(ui, max_size / height);
        }
    }

    fn texture_preview(&self, ui: &mut Ui, tex: &Texture) {
        match tex {
            Texture::Colour(c) => {
                ui.label("Colour");
                let colour: Color32 = egui::Rgba::from_rgb(c.x, c.y, c.z).into();
                show_color(ui, colour, ui.available_size_before_wrap());
            }
            Texture::Image(img) => {
                ui.label("Image");
                ui.label("example.png")
                    .on_hover_ui(|ui| self.image_preview(ui, img, 250.0));
            }
        }
    }

    fn editor(&mut self, ui: &mut Ui, state: &mut TextureEditorState, tex: &mut Texture) {
        ui.vertical(|ui| {
            ui.style_mut().wrap = Some(false);
            ui.radio_value(&mut state.choosing_image, false, "Colour");
            ui.radio_value(&mut state.choosing_image, true, "Image");
        });
        if state.choosing_image {
            if let Some(ref edited_image_id) = state.edited_image_id {
                self.image_preview(ui, edited_image_id, 100.0);
            }
            if ui.button("Open file...").clicked() {
                state.edited_image_id = rfd::FileDialog::new()
                    .pick_file()
                    .and_then(|path| load_image(path.display().to_string()))
                    .map(|img| self.add_image(img));
            }
        } else {
            ui.color_edit_button_rgb(&mut state.edited_rgb);
        }
        ui.vertical(|ui| {
            ui.style_mut().wrap = Some(false);
            let save = ui.button("Save");
            let cancel = ui.button("Cancel");
            if save.clicked() {
                if !state.choosing_image {
                    *tex = Texture::Colour(state.edited_rgb.into());
                } else if let Some(img_id) = state.edited_image_id {
                    *tex = Texture::Image(img_id);
                }
            }
            if cancel.clicked() || save.clicked() {
                state.edited_id = None;
            }
        });
    }
}

impl View for TextureEditor {
    fn title(&self) -> &str {
        "Textures"
    }

    fn ui(&mut self, ui: &mut Ui) {
        let mut editor_state = take(&mut self.editor_state);
        let mut textures = take(&mut self.textures);

        let mut tex_iter = textures.iter_mut().peekable();

        grid(ui, "Textures1", 4, true).show(ui, |ui| {
            while let Some((&id, tex)) = tex_iter.peek() {
                ui.label(format!("Texture {}", id));
                self.texture_preview(ui, tex);
                if editor_state.edited_id == None {
                    if ui.button("Edit").clicked() {
                        editor_state.setup(id, tex);
                    }
                } else {
                    ui.label("");
                }
                if let Some(edited_id) = editor_state.edited_id {
                    if edited_id == id {
                        break;
                    }
                }
                tex_iter.next();
                ui.end_row();
            }
        });

        if let Some((_id, tex)) = tex_iter.next() {
            self.editor(ui, &mut editor_state, tex);
        }

        grid(ui, "Textures2", 4, true).show(ui, |ui| {
            for (id, tex) in tex_iter {
                ui.label(format!("Texture {}", id));
                self.texture_preview(ui, tex);
                if editor_state.edited_id == None {
                    if ui.button("Edit").clicked() {
                        editor_state.setup(*id, tex);
                    }
                } else {
                    ui.label("");
                }
                ui.end_row();
            }
        });

        self.editor_state = editor_state;
        self.textures = textures;
    }
}

pub struct OldTextureEditor {
    scene: Rc<RefCell<Scene>>,
    // Editor
    edited_id: Option<Id<Texture>>,
    edited_rgb: [f32; 3],
    edited_image_id: Option<Id<RgbImage>>,
    choosing_colour: bool,

    // RetaindedImage storage. Possibly moved into its own window.
    images: RefCell<IndexMap<Id<RgbImage>, RetainedImage>>,
    fallback: RetainedImage,
}

impl OldTextureEditor {
    pub fn new(scene: Rc<RefCell<Scene>>) -> Self {
        OldTextureEditor {
            scene,
            edited_id: None,
            edited_rgb: [0f32, 0f32, 0f32],
            edited_image_id: None,

            choosing_colour: true,
            images: RefCell::new(IndexMap::new()),
            fallback: RetainedImage::from_color_image("Fallback", ColorImage::example()),
        }
    }
}

impl View for OldTextureEditor {
    fn title(&self) -> &str {
        "Texture Editor"
    }

    fn ui(&mut self, ui: &mut Ui) {
        egui::Grid::new(ui.auto_id_with("textures"))
            .num_columns(4)
            .striped(true)
            .show(ui, |ui| {
                for (id, tex) in self.scene.borrow().textures.borrow_mut().iter_mut() {
                    ui.label(format!("Texture {}", id));
                    if let Texture::Colour(c) = tex {
                        ui.label("Colour");
                        let colour: Color32 = egui::Rgba::from_rgb(c.x, c.y, c.z).into();
                        show_color(ui, colour, ui.available_size_before_wrap());
                    } else if let Texture::Image(img) = tex {
                        ui.label("Image");
                        ui.label(RichText::new("example.png").italics().underline())
                            .on_hover_ui(|ui| {
                                let images_ref = self.images.borrow();
                                let ret_img = images_ref.get(img).unwrap_or(&self.fallback);
                                ret_img.show_size(ui, Vec2::new(100f32, 100f32));
                            });
                    }
                    if self.edited_id == None {
                        if ui.button("Edit").clicked() {
                            self.edited_id = Some(*id);
                            if let Texture::Colour(c) = tex {
                                self.choosing_colour = true;
                                self.edited_rgb = [c.x, c.y, c.z];
                                self.edited_image_id = None;
                            } else if let Texture::Image(img) = tex {
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
                                if let Some(ref edited_image_id) = self.edited_image_id {
                                    let images_ref = self.images.borrow();
                                    let ret_img =
                                        images_ref.get(edited_image_id).unwrap_or(&self.fallback);
                                    ret_img.show_size(ui, Vec2::new(100f32, 100f32));
                                }
                                if ui.button("Open file...").clicked() {
                                    self.edited_image_id = rfd::FileDialog::new()
                                        .pick_file()
                                        .and_then(|path| load_image(path.display().to_string()))
                                        .map(|img| {
                                            let img = Arc::new(img);
                                            let img_id = self.scene.borrow().add_image(img.clone());
                                            self.images
                                                .borrow_mut()
                                                .insert(img_id, Image(img.as_ref()).into());
                                            img_id
                                        });
                                }
                            }
                            ui.vertical(|ui| {
                                ui.style_mut().wrap = Some(false);
                                let save = ui.button("Save");
                                let cancel = ui.button("Cancel");
                                if save.clicked() {
                                    if self.choosing_colour {
                                        *tex = Texture::Colour(self.edited_rgb.into());
                                    } else if let Some(img_id) = self.edited_image_id {
                                        *tex = Texture::Image(img_id);
                                    }
                                }
                                if cancel.clicked() || save.clicked() {
                                    self.edited_id = None;
                                }
                            });
                        }
                    }
                    ui.end_row();
                }
            });
    }
}

fn grid(ui: &mut Ui, name: &'static str, num_columns: usize, stripped: bool) -> egui::Grid {
    egui::Grid::new(ui.auto_id_with(name))
        .num_columns(num_columns)
        .striped(stripped)
}
