use std::rc::Rc;

use cgmath::point3;
use image::RgbImage;
use rusty_trace::camera::Camera;
use rusty_trace::gui::{start, Gui, WindowDimensions};
use rusty_trace::hittable::{Sphere, HittableVec};
use rusty_trace::{render, Colour};
use rusty_trace::material::Lambertian;

static WIN_DIMENSIONS: WindowDimensions = WindowDimensions {
    width: 640,
    height: 360,
}; // small resolution to make debug rendering faster
static WIN_TITLE: &str = "Rusty Trace";

fn main() -> Result<(), String> {
    let (width, height) = (WIN_DIMENSIONS.width, WIN_DIMENSIONS.height);
    let mut image = RgbImage::new(width, height);
    
    let camera = Camera::from_aspect_ratio(width as f64 / height as f64);

    let mut scene = HittableVec::new();
    let material = Rc::new(Lambertian { albedo: Colour::new(0.8, 0.8, 0.8) });
    scene.push(Box::new(Sphere::new(point3(0.0, 0.0, -1.0), 0.5, material.clone())));
    scene.push(Box::new(Sphere::new(point3(0.0, -20.5, -1.0), 20.0, material)));

    render(&mut image, &camera, &scene, Colour::new(0.8, 0.8, 0.9), 50);

    let mut gui = Gui::default();
    gui.add_image(image)?;
    start(gui, WIN_DIMENSIONS, WIN_TITLE).unwrap();
    Ok(())
}
