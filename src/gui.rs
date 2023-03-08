extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::{Sdl, VideoSubsystem, EventPump};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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
    should_close: bool,
}

impl Gui {
    pub fn init(context: Sdl, dimensions: WindowDimensions, title: &str) -> Result<Gui, String> {
        let video = context.video()?;
        let event_pump = context.event_pump()?;
        let window = video.window(title, dimensions.width, dimensions.height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Gui { context, video, event_pump, canvas, dimensions, should_close: false })
    }

    pub fn mainloop(&mut self) {
        'main: while !self.should_close {
            // poll events
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        self.should_close = true;
                        break 'main;
                    },
                    _ => {}
                }
            }

            self.canvas.set_draw_color(Color::WHITE);
            self.canvas.clear();
            self.canvas.present();
        }
    }
}
