#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::sync::mpsc;
use std::thread::JoinHandle;

use eframe::egui;
use egui_extras::RetainedImage;
use egui::ColorImage;
use image::RgbImage;

#[derive(Clone, Copy)]
pub struct WindowDimensions {
    pub width: u32,
    pub height: u32,
}

struct ImageThreadState {
    thread_nr: usize,
    title: String,
    image: RetainedImage,
}

impl ImageThreadState {
    fn new(thread_nr: usize, image: RgbImage) -> Self {
        let title = format!("Render: {thread_nr}");
        Self {
            thread_nr,
            title,
            image: RetainedImage::from_color_image(
                "render",
                ColorImage::from_rgb([image.dimensions().0 as usize, image.dimensions().1 as usize], image.as_raw())
            )
        }
    }

    fn show(&mut self, ctx: &egui::Context) {
        let pos = egui::pos2(16.0, 128.0 * (self.thread_nr as f32 + 1.0));
        egui::Window::new(&self.title)
            .default_pos(pos)
            .show(ctx, |ui| {
                self.image.show(ui);
            });
    }
}

fn new_worker(
    mut state: ImageThreadState,
    on_done_tx: mpsc::SyncSender<()>,
) -> (JoinHandle<()>, mpsc::SyncSender<egui::Context>) {
    let (show_tx, show_rc) = mpsc::sync_channel(0);
    let handle = std::thread::Builder::new()
        .name(format!("EguiPanelWorker {}", state.thread_nr))
        .spawn(move || {
            while let Ok(ctx) = show_rc.recv() {
                state.show(&ctx);
                let _ = on_done_tx.send(());
            }
        })
        .expect("failed to spawn thread");
    (handle, show_tx)
}

pub struct Gui {
    threads: Vec<(JoinHandle<()>, mpsc::SyncSender<egui::Context>)>,
    on_done_tx: mpsc::SyncSender<()>,
    on_done_rc: mpsc::Receiver<()>,
}

impl Default for Gui {
    fn default() -> Self {
        let threads = Vec::with_capacity(3);
        let (on_done_tx, on_done_rc) = mpsc::sync_channel(0);

        Self {
            threads,
            on_done_tx,
            on_done_rc,
        }
    }
}

impl Gui {
    pub fn add_image(&mut self, image: RgbImage) -> Result<(), String> {
        // self.images.push(RetainedImage::from_color_image(
        //     "render",
        //     ColorImage::from_rgb([image.dimensions().0 as usize, image.dimensions().1 as usize], image.as_raw())
        // ));

        let thread_nr = self.threads.len();
        self.threads.push(
            new_worker(ImageThreadState::new(thread_nr, image), self.on_done_tx.clone())
        );
        Ok(())
    }
}

pub fn start(gui: Gui, dimensions: WindowDimensions, title: &str) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(dimensions.width as f32, dimensions.height as f32)),
        ..Default::default()
    };

    eframe::run_native(
        title,
        options,
        Box::new(|_cc| Box::new(gui)),
    )
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for (_handle, show_tx) in &self.threads {
                let _ = show_tx.send(ctx.clone());
            }

            for _ in 0..self.threads.len() {
                let _ = self.on_done_rc.recv();
            }

        });
    }
}
