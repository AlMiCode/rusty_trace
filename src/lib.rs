use cgmath::{vec3, InnerSpace};
use image::Rgb;
use shapes::{ShapeEnum, HitObject};

pub mod gui;
pub mod scene;
pub mod shapes;

pub type Float = f64;
pub type Point3 = cgmath::Point3<Float>;
pub type Vector3 = cgmath::Vector3<Float>;
pub type Colour = cgmath::Vector3<Float>;

pub struct Camera {
    pub viewport_width: Float,
    pub viewport_height: Float,
    pub focal_length: Float,
    pub origin: Point3,
    pub horizontal: Vector3,
    pub vertical: Vector3,
    pub lower_left_corner: Point3,
}

impl Default for Camera {
    fn default() -> Camera {
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

pub struct Ray {
    origin: Point3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Ray { origin, direction }
    }
    pub fn at(&self, t: Float) -> Point3 {
        self.origin + self.direction * t
    }
}

pub fn ray_colour(ray: Ray, hittable: &Vec<ShapeEnum>) -> Colour {
    let mut t = -1.0;
    for shape in hittable {
        let t1 = match shape {
            ShapeEnum::Sphere(s) => s.hit(&ray),
        };
        if t1 == -1.0 { continue }
        if t == -1.0 { t = t1 } else if t1 < t { t = t1 }
    }
    if t > 0.0 {
        let normal_vec = ray.at(t) - Vector3::new(0.0,0.0,-1.0);
        let normal_vec = Vector3::new(normal_vec.x, normal_vec.y, normal_vec.z).normalize();
        0.5 * (normal_vec + Vector3::new(1.0, 1.0, 1.0))
    } else {
        let unit_dir = ray.direction.normalize();
        let t = 0.5 * (unit_dir.y + 1.0);
        (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0)
    }
}

pub fn vec_to_rgb(vec: Colour) -> Rgb<u8> {
    Rgb(vec.map(|n| (n.clamp(0.0, 1.0) * 255.0) as u8).into())
}
