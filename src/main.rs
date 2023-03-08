extern crate sdl2;
extern crate cgmath;

pub mod ray;
pub mod gui;

use gui::Gui;
use gui::WindowDimensions;

static WIN_DIMENSIONS: WindowDimensions = WindowDimensions { width: 1280, height: 720 };
static WIN_TITLE: &str = "Rusty_trace";

fn main() -> Result<(), String> {
    // initialize gui object
    let mut gui: Gui;
    {
        let sdl_context = sdl2::init()?;
        gui = Gui::init(sdl_context, WIN_DIMENSIONS, WIN_TITLE)?;
    }

    gui.mainloop();

    Ok(())
}
