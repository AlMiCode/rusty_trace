use std::{mem::take, sync::mpsc::Receiver};

use crate::render::RenderedImage;

use super::{image_to_retained, View};
use egui::Ui;
use egui_extras::RetainedImage;
use image::DynamicImage;

pub enum RenderedImageView {
    Waiting {
        title: String,
        rx: Receiver<RenderedImage>,
    },
    Ready {
        title: String,
        colour: RetainedImage,
        albedo: RetainedImage,
        normal: RetainedImage,
        denoised: RetainedImage,
        viewed_option: u8,
    },
}

impl RenderedImageView {
    pub fn new(title: String, rx: Receiver<RenderedImage>) -> Self {
        RenderedImageView::Waiting { title, rx }
    }
}

impl View for RenderedImageView {
    fn title(&self) -> &str {
        match self {
            Self::Waiting { title, rx: _ } => &title,
            Self::Ready {
                title,
                colour: _,
                albedo: _,
                normal: _,
                denoised: _,
                viewed_option: _,
            } => &title,
        }
    }

    fn ui(&mut self, ui: &mut Ui) {
        match self {
            Self::Waiting { title, rx } => {
                if let Ok(mut img) = rx.try_recv() {
                    let colour =
                        image_to_retained(&DynamicImage::ImageRgb32F(img.colour).into_rgb8());
                    let albedo =
                        image_to_retained(&DynamicImage::ImageRgb32F(img.albedo).into_rgb8());
                    for n in img.normal.as_mut().iter_mut() {
                        *n = (*n + 1.0) / 2.0;
                    }
                    let normal =
                        image_to_retained(&DynamicImage::ImageRgb32F(img.normal).into_rgb8());
                    let denoised =
                        image_to_retained(&DynamicImage::ImageRgb32F(img.denoised).into_rgb8());
                    *self = RenderedImageView::Ready {
                        title: take(title),
                        colour,
                        albedo,
                        normal,
                        denoised,
                        viewed_option: 0u8,
                    }
                }
                ui.spinner();
            }
            Self::Ready {
                title: _,
                colour,
                albedo,
                normal,
                denoised,
                viewed_option,
            } => {
                match viewed_option {
                    0 => colour.show_max_size(ui, ui.available_size()),
                    1 => albedo.show_max_size(ui, ui.available_size()),
                    2 => normal.show_max_size(ui, ui.available_size()),
                    3 => denoised.show_max_size(ui, ui.available_size()),
                    _ => unreachable!(),
                };
                ui.horizontal_wrapped(|ui| {
                    if ui.selectable_label(*viewed_option == 0u8, "Raw").clicked() {
                        *viewed_option = 0u8;
                    }
                    if ui
                        .selectable_label(*viewed_option == 1u8, "Albedo")
                        .clicked()
                    {
                        *viewed_option = 1u8;
                    }
                    if ui
                        .selectable_label(*viewed_option == 2u8, "Normal")
                        .clicked()
                    {
                        *viewed_option = 2u8;
                    }
                    if ui
                        .selectable_label(*viewed_option == 3u8, "Denoised")
                        .clicked()
                    {
                        *viewed_option = 3u8;
                    }
                });
                ui.horizontal_wrapped(|ui| {
                    let size = colour.size();
                    ui.label(format!("Size: {}x{}", size[0], size[1]));
                    if ui.button("Save").clicked() {
                        eprintln!("TODO: Save image");
                    }
                });
            }
        }
    }
}
