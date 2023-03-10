use rusty_trace::gui::{Gui, WindowDimensions};
use rusty_trace::scene::Scene;
use rusty_trace::{Camera, Point3, Vector3};

static WIN_DIMENSIONS: WindowDimensions = WindowDimensions {
    width: 1280,
    height: 720,
};
static WIN_TITLE: &str = "Rusty_trace";

fn main() -> Result<(), String> {
    // initialize gui object
    let mut gui = Gui::init(WIN_DIMENSIONS, WIN_TITLE)?;

    // let aspect_ratio: f32 = (WIN_DIMENSIONS.width / WIN_DIMENSIONS.height) as f32;
    let scene = Scene::new(Camera::default(), WIN_DIMENSIONS);

    gui.set_scene(scene);
    gui.mainloop()?;

    Ok(())
}
