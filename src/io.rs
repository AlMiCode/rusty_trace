pub fn load_image(filename: String) -> Option<image::RgbImage> {
    try_load_image(&filename.into()).ok()
}

pub fn try_load_image(filename: &std::path::PathBuf) -> image::ImageResult<image::RgbImage> {
    Ok(image::io::Reader::open(filename)?.decode()?.to_rgb8())
}
