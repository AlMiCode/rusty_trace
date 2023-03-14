use rusty_trace::gui::{start, Gui, WindowDimensions};

static WIN_DIMENSIONS: WindowDimensions = WindowDimensions {
    width: 640,
    height: 360,
}; // small resolution to make debug rendering faster
static WIN_TITLE: &str = "Rusty Trace";

fn main() -> Result<(), String> {
    let gui = Gui::default();
    start(gui, WIN_DIMENSIONS, WIN_TITLE).unwrap();
    Ok(())
}
