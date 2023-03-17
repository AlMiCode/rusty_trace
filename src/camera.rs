use crate::{random_vec_in_disc, Point3, Ray, Vector3};
use cgmath::{point3, vec3, InnerSpace};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vector3,
    vertical: Vector3,

    w: Vector3,
    u: Vector3,
    v: Vector3,

    lens_radius: f64,
    pub settings: CameraSettings,
}

#[derive(Copy, Clone)]
pub struct CameraSettings {
    pub look_from: Point3,
    pub look_at: Point3,
    pub up_vec: Vector3,
    pub fov: f64,
    pub aspect_ratio: f64,
    pub aperture: f64,
}

impl Default for CameraSettings {
    fn default() -> Self {
        CameraSettings {
            look_from: point3(0.0, 0.0, 0.0),
            look_at: point3(0.0, 0.0, -1.0),
            up_vec: vec3(0.0, 1.0, 0.0),
            fov: 45.0,
            aspect_ratio: 1.0,
            aperture: 1.0 / 16.0,
        }
    }
}

impl Camera {
    pub fn new(s: CameraSettings) -> Self {
        let theta = s.fov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = s.aspect_ratio * viewport_height;
        let focus_dist = (s.look_from - s.look_at).magnitude();

        let w = (s.look_from - s.look_at).normalize();
        let u = s.up_vec.cross(w);
        let v = w.cross(u);

        let origin = s.look_from;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

        let lens_radius = s.aperture / 2.0;
        println!("{}", lens_radius);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            w,
            u,
            v,
            lens_radius,
            settings: s,
        }
    }

    pub fn update(&mut self) {
        let updated = Camera::new(self.settings);
        *self = updated;
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = random_vec_in_disc() * self.lens_radius;
        let offset = (self.u * rd.x) + (self.v * rd.y);
        let direction = (self.lower_left_corner + self.horizontal * u + self.vertical * v
            - self.origin
            - offset)
            .normalize();
        Ray {
            origin: self.origin + offset,
            direction,
        }
    }
}
