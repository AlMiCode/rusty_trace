pub fn try_open(filename: &std::path::PathBuf) -> image::ImageResult<image::RgbImage> {
    Ok(image::io::Reader::open(filename)?.decode()?.into_rgb8())
}