use std::{mem::take, sync::mpsc::Receiver};

use crate::{
    gui::image_storage::IMAGE_STORAGE,
    render::{texture::Image, RenderedImage},
};

use super::View;
use egui::Ui;

use image::{DynamicImage, Rgb32FImage, RgbImage};

pub enum RenderedImageView {
    Waiting {
        title: String,
        rx: Receiver<RenderedImage>,
    },
    Ready {
        title: String,
        colour: Image,
        albedo: Image,
        normal: Image,
        denoised: Image,
        viewed_option: u8,
        size: (u32, u32),
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
                size: _,
            } => &title,
        }
    }

    fn ui(&mut self, ui: &mut Ui) {
        match self {
            Self::Waiting { title, rx } => {
                if let Ok(mut img) = rx.try_recv() {
                    fn imgf32_to_imgu8(img: Rgb32FImage) -> RgbImage {
                        DynamicImage::ImageRgb32F(img).into_rgb8()
                    }

                    let size = img.colour.dimensions();

                    let colour = Image::new(imgf32_to_imgu8(img.colour));
                    let albedo = Image::new(imgf32_to_imgu8(img.albedo));
                    for n in img.normal.as_mut().iter_mut() {
                        *n = (*n + 1.0) / 2.0;
                    }
                    let normal = Image::new(imgf32_to_imgu8(img.normal));
                    let denoised = Image::new(imgf32_to_imgu8(img.denoised));
                    *self = RenderedImageView::Ready {
                        title: take(title),
                        colour,
                        albedo,
                        normal,
                        denoised,
                        viewed_option: 0u8,
                        size,
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
                size,
            } => {
                fn show_image(ui: &mut Ui, img: &Image) {
                    IMAGE_STORAGE.with_retained(img, |image| {
                        image.show_max_size(ui, ui.available_size());
                    });
                }
                match viewed_option {
                    0 => show_image(ui, &colour),
                    1 => show_image(ui, &albedo),
                    2 => show_image(ui, &normal),
                    3 => show_image(ui, &denoised),
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
                    ui.label(format!("Size: {}x{}", size.0, size.1));
                    if ui.button("Save").clicked() {
                        eprintln!("TODO: Save image");
                    }
                });
            }
        }
    }
}
