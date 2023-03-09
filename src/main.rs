use rusty_trace::gui::{Gui, WindowDimensions};

static WIN_DIMENSIONS: WindowDimensions = WindowDimensions { width: 1280, height: 720 };
static WIN_TITLE: &str = "Rusty_trace";

fn main() -> Result<(), String> {
    // initialize gui object
    let mut gui = Gui::init(WIN_DIMENSIONS, WIN_TITLE)?;

    gui.mainloop();

    Ok(())
}
