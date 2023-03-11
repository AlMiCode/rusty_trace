use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::{EventPump, Sdl, VideoSubsystem};

use image::RgbImage;

#[derive(Clone, Copy)]
pub struct WindowDimensions {
    pub width: u32,
    pub height: u32,
}

#[allow(dead_code)]
pub struct Gui {
    context: Sdl,
    video: VideoSubsystem,
    event_pump: EventPump,
    canvas: WindowCanvas,
    dimensions: WindowDimensions,
    image: Option<RgbImage>,
    should_close: bool,
}

impl Gui {
    pub fn init(dimensions: WindowDimensions, title: &str) -> Result<Gui, String> {
        let context = sdl2::init()?;
        let video = context.video()?;
        let event_pump = context.event_pump()?;
        let window = video
            .window(title, dimensions.width, dimensions.height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Gui {
            context,
            video,
            event_pump,
            canvas,
            dimensions,
            should_close: false,
            image: None,
        })
    }

    pub fn set_image(&mut self, scene: RgbImage) {
        self.image = Some(scene);
    }

    pub fn mainloop(&mut self) -> Result<(), String> {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::RGB24,
                self.dimensions.width,
                self.dimensions.height,
            )
            .map_err(|e| e.to_string())?;

        'main: while !self.should_close {
            // poll events
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        self.should_close = true;
                        break 'main;
                    }
                    _ => {}
                }
            }

            self.canvas.set_draw_color(Color::WHITE);
            self.canvas.clear();

            if let Some(image) = self.image.as_ref() {
                texture.with_lock(None, |buffer: &mut [u8], _pitch: usize| {
                    buffer.clone_from_slice(image.as_raw().as_slice());
                })?;
                self.canvas.copy(
                    &texture,
                    None,
                    Some(Rect::new(
                        0,
                        0,
                        self.dimensions.width,
                        self.dimensions.height,
                    )),
                )?;
            }

            self.canvas.present();
        }
        Ok(())
    }

    pub fn show_image(&mut self, image: RgbImage) -> Result<(), String> {
        self.set_image(image);
        self.mainloop()
    }
}
