pub struct Camera {
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub focal_length: f64,
    pub origin: Point3,
    pub horizontal: Vector3,
    pub vertical: Vector3,
    pub lower_left_corner: Point3,
}

impl Default for Camera {
    fn default() -> Self {
        let viewport_height = 2.0;
        let viewport_width = (1280.0 / 720.0) * 2.0;
        let focal_length = 1.0;
        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);
        Camera {
            viewport_width,
            viewport_height,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}