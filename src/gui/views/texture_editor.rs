use std::{mem::take};

use egui::{color_picker::show_color, Color32, Ui};


use crate::{render::{
    texture::{Texture, Image},
}, vec_repo::{VecRepo, Id}, gui::image_storage::IMAGE_STORAGE};

use super::{grid, View};
use crate::io;

#[derive(Default)]
struct TextureEditorState {
    edited_id: Option<usize>,
    choosing_image: bool,
    edited_rgb: [f32; 3],
    edited_image: Option<Image>,
}

impl TextureEditorState {
    fn setup(&mut self, id: usize, tex: &Texture) {
        self.edited_id = Some(id);
        match tex {
            Texture::Colour(c) => {
                self.choosing_image = false;
                self.edited_rgb = [c.x, c.y, c.z];
                self.edited_image = None;
            }
            Texture::Image(img) => {
                self.choosing_image = true;
                self.edited_image = Some(img.clone());
            }
        }
    }

    fn show_editor(&mut self, ui: &mut Ui, tex: &mut Texture) {
        ui.vertical(|ui| {
            ui.style_mut().wrap = Some(false);
            ui.radio_value(&mut self.choosing_image, false, "Colour");
            ui.radio_value(&mut self.choosing_image, true, "Image");
        });
        if self.choosing_image {
            if let Some(ref edited_image) = self.edited_image {
                image_preview(ui, edited_image, 100.0);
            }
            if ui.button("Open file...").clicked() {
                self.edited_image = rfd::FileDialog::new()
                    .pick_file()
                    .and_then(|path| io::try_open(&path).ok())
                    .map(|image| {
                        let image = Image::new(image);
                        IMAGE_STORAGE.add_retained(&image);
                        image
                    });
            }
        } else {
            ui.color_edit_button_rgb(&mut self.edited_rgb);
        }
        ui.vertical(|ui| {
            ui.style_mut().wrap = Some(false);
            let save = ui.button("Save");
            let cancel = ui.button("Cancel");
            if save.clicked() {
                if !self.choosing_image {
                    *tex = Texture::Colour(self.edited_rgb.into());
                } else if let Some(ref img) = self.edited_image {
                    *tex = img.clone().into();
                }
            }
            if cancel.clicked() || save.clicked() {
                self.edited_id = None;
            }
        });
    }
}

#[derive(Default)]
pub struct TextureEditor {
    editor_state: TextureEditorState,
    textures: VecRepo<Texture>,
}


impl From<VecRepo<Texture>> for TextureEditor {
    fn from(value: VecRepo<Texture>) -> Self {
        Self {
            editor_state: TextureEditorState::default(),
            textures: value,
        }
    }
}

impl TextureEditor {
    pub fn get_repo(&self) -> &VecRepo<Texture> {
        &self.textures
    }

    pub fn texture_picker(&self, ui: &mut Ui, tex_id: &mut Id<Texture>) {
        egui::ComboBox::from_label("")
            .selected_text(format!("Texture {}", tex_id))
            .show_ui(ui, |ui| {
                ui.selectable_value(tex_id, Id::default(), "Default");
                for (option, _tex) in self.textures.iter().enumerate().skip(1) {
                    ui.selectable_value(
                        tex_id,
                        Id::new(option as u32),
                        format!("Texture {}", option),
                    );
                }
            });
        texture_preview(ui, self.textures.get(*tex_id), false);
    }
}

impl View for TextureEditor {
    fn title(&self) -> &str {
        "Textures"
    }

    fn ui(&mut self, ui: &mut Ui) {
        let Self { editor_state, textures } = self;
        let mut tex_iter = textures.iter_mut().enumerate().peekable();

        grid(ui, "Textures1", 4, true).show(ui, |ui| {
            while let Some((id, tex)) = tex_iter.peek() {
                ui.label(format!("Texture {}", id));
                texture_preview(ui, tex, true);
                if editor_state.edited_id == None {
                    if ui.button("Edit").clicked() {
                        editor_state.setup(*id, tex);
                    }
                } else {
                    ui.label("");
                }
                if let Some(edited_id) = editor_state.edited_id {
                    if edited_id == *id {
                        break;
                    }
                }
                tex_iter.next();
                ui.end_row();
            }
        });

        if let Some((_id, tex)) = tex_iter.next() {
            editor_state.show_editor(ui, tex);
        }

        grid(ui, "Textures2", 4, true).show(ui, |ui| {
            for (id, tex) in tex_iter {
                ui.label(format!("Texture {}", id));
                texture_preview(ui, tex, true);
                if self.editor_state.edited_id == None {
                    if ui.button("Edit").clicked() {
                        self.editor_state.setup(id, tex);
                    }
                } else {
                    ui.label("");
                }
                ui.end_row();
            }
        });
    }
}

fn texture_preview(ui: &mut Ui, tex: &Texture, show_type: bool) {
    match tex {
        Texture::Colour(c) => {
            if show_type {
                ui.label("Colour");
            }
            let colour: Color32 = egui::Rgba::from_rgb(c.x, c.y, c.z).into();
            show_color(ui, colour, ui.available_size_before_wrap());
        }
        Texture::Image(img) => {
            if show_type {
                ui.label("Image");
            }
            ui.label("example.png")
                .on_hover_ui(|ui| image_preview(ui, img, 250.0));
        }
    }
}

fn image_preview(ui: &mut Ui, img: &Image, max_size: f32) {
    IMAGE_STORAGE.with_retained(img, |image| {
        let [width, height] = image.size();
        let (width, height) = (width as f32, height as f32);
        if width >= height {
            image.show_scaled(ui, max_size / width);
        } else {
            image.show_scaled(ui, max_size / height);
        }
    });
}
