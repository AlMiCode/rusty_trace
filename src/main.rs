use rusty_trace::gui::{start, Gui, WindowDimensions};

static WIN_DIMENSIONS: WindowDimensions = WindowDimensions {
    width: 1280,
    height: 720,
}; // small resolution to make debug rendering faster
static WIN_TITLE: &str = "Rusty Trace";

fn main() -> Result<(), String> {
    let gui = Gui::default();
    start(gui, WIN_DIMENSIONS, WIN_TITLE).unwrap();
    Ok(())
}
