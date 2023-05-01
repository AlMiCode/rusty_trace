use std::{collections::HashMap, mem::take, vec};

use egui::{color_picker::show_color, Color32, ColorImage, Ui};
use egui_extras::RetainedImage;

use crate::render::{
    repo::{Id, VecRepo},
    texture::Texture,
};

use super::{grid, image_to_retained, View};
use crate::io;

#[derive(Default)]
struct TextureEditorState {
    edited_id: Option<usize>,
    choosing_image: bool,
    edited_rgb: [f32; 3],
    edited_image: Option<io::Image>,
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
}

pub struct TextureEditor {
    editor_state: TextureEditorState,
    textures: Vec<Texture>,
    retained_images: HashMap<io::Image, RetainedImage>,
}

impl Default for TextureEditor {
    fn default() -> Self {
        Self {
            editor_state: TextureEditorState::default(),
            textures: vec![Texture::default()],
            retained_images: HashMap::new(),
        }
    }
}

impl From<VecRepo<Texture>> for TextureEditor {
    fn from(value: VecRepo<Texture>) -> Self {
        Self {
            editor_state: TextureEditorState::default(),
            textures: value.into(),
            retained_images: HashMap::new(),
        }
    }
}

impl TextureEditor {
    pub fn get_repo(&self) -> VecRepo<Texture> {
        self.textures.clone().into()
    }

    pub fn texture_picker(&self, ui: &mut Ui, tex_id: &mut Id<Texture>) {
        egui::ComboBox::from_label("")
            .selected_text(format!("Texture {}", tex_id))
            .show_ui(ui, |ui| {
                ui.selectable_value(tex_id, Id::default(), "Default");
                for (option, _tex) in self.textures.iter().enumerate().skip(1) {
                    ui.selectable_value(
                        tex_id,
                        (option as u32).into(),
                        format!("Texture {}", option),
                    );
                }
            });
        self.texture_preview(ui, &self.textures[tex_id.id as usize], false);
    }

    fn add_image(&mut self, image: io::Image) {
        self.retained_images
            .insert(image.clone(), image_to_retained(&image));
    }

    fn image_preview(&self, ui: &mut Ui, img: &io::Image, max_size: f32) {
        let fallback: RetainedImage =
            RetainedImage::from_color_image("Fallback", ColorImage::example());

        let image = self.retained_images.get(img).unwrap_or(&fallback);

        let [width, height] = image.size();
        let (width, height) = (width as f32, height as f32);
        if width >= height {
            image.show_scaled(ui, max_size / width);
        } else {
            image.show_scaled(ui, max_size / height);
        }
    }

    fn texture_preview(&self, ui: &mut Ui, tex: &Texture, show_type: bool) {
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
            if let Some(ref edited_image) = state.edited_image {
                self.image_preview(ui, edited_image, 100.0);
            }
            if ui.button("Open file...").clicked() {
                state.edited_image = rfd::FileDialog::new()
                    .pick_file()
                    .and_then(|path| io::Image::try_open(&path).ok())
                    .map(|image| {
                        self.add_image(image.clone());
                        image
                    });
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
                } else if let Some(ref img) = state.edited_image {
                    *tex = img.clone().into();
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

        let mut tex_iter = textures.iter_mut().enumerate().peekable();

        grid(ui, "Textures1", 4, true).show(ui, |ui| {
            while let Some((id, tex)) = tex_iter.peek() {
                ui.label(format!("Texture {}", id));
                self.texture_preview(ui, tex, true);
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
            self.editor(ui, &mut editor_state, tex);
        }

        grid(ui, "Textures2", 4, true).show(ui, |ui| {
            for (id, tex) in tex_iter {
                ui.label(format!("Texture {}", id));
                self.texture_preview(ui, tex, true);
                if editor_state.edited_id == None {
                    if ui.button("Edit").clicked() {
                        editor_state.setup(id, tex);
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
