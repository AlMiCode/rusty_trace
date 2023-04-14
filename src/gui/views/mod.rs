use egui::Ui;

mod texture_editor;
pub use texture_editor::TextureEditor;

mod preview;

pub trait View {
    fn ui(&mut self, ui: &mut Ui);
}
