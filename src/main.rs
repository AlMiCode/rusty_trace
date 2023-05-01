use rusty_trace::gui::Gui;

const WIN_TITLE: &str = "Rusty Trace";

fn main() -> Result<(), eframe::Error> {
    Gui::default().start(WIN_TITLE)
}
