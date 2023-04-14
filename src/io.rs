use image::RgbImage;

pub fn load_image(filename: String) -> Option<RgbImage> {
    let reader = match image::io::Reader::open(&filename) {
        Err(_) => {
            eprintln!("Could not read");
            return None;
        }
        Ok(r) => r,
    };
    let dyn_img = match reader.decode() {
        Err(_) => {
            eprintln!("Could not decode");
            return None;
        }
        Ok(img) => img,
    };
    Some(dyn_img.to_rgb8())
}
