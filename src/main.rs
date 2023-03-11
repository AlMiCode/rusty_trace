use image::RgbImage;
use rusty_trace::camera::Camera;
use rusty_trace::gui::{Gui, WindowDimensions};
use rusty_trace::hittable::{Sphere, HittableVec};
use rusty_trace::{Point3, render};

static WIN_DIMENSIONS: WindowDimensions = WindowDimensions {
    width: 1280,
    height: 720,
};
static WIN_TITLE: &str = "Rusty Trace";

fn main() -> Result<(), String> {
    let mut image = RgbImage::new(WIN_DIMENSIONS.width, WIN_DIMENSIONS.height);
    
    let camera = Camera::default();

    let mut scene = HittableVec::new();
    scene.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));

    render(&mut image, &camera, &scene);

    // initialize gui object
    let mut gui = Gui::init(WIN_DIMENSIONS, WIN_TITLE)?;
    gui.show_image(image)
}
