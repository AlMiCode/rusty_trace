use rusty_trace::gui::Gui;

static WIN_TITLE: &str = "Rusty Trace";

fn main() -> Result<(), eframe::Error> {
    Gui::default().start(WIN_TITLE)
}
