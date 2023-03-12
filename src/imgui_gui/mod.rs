use imgui::Condition;

mod support;

#[derive(Copy, Clone)]
pub struct WindowDimensions {
    pub width: u32,
    pub height: u32,
}

pub struct Gui {
    // pub GuiElements: Vec<GuiElement>,
}

impl Gui {
    pub fn start(&self, dimensions: WindowDimensions, title: &str) -> Result<(), String> {
        let system = support::init((dimensions.width, dimensions.height), title);

        system.main_loop(move |_, ui| {
            ui.window("Hello")
                .size([dimensions.width as f32,dimensions.height as f32], Condition::Always)
                .position([0.0,0.0], Condition::Always)
                .no_decoration()
                .build(|| {
                    ui.text_wrapped("Hello");
                });
        });
        Ok(())
    }
}
