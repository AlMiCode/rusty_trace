use crate::render::{random_vec_in_disc, Point3, Ray, Vector3};
use cgmath::{point3, vec3, InnerSpace};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vector3,
    vertical: Vector3,

    _w: Vector3,
    u: Vector3,
    v: Vector3,

    lens_radius: f64,
    pub settings: CameraSettings,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CameraSettings {
    pub look_from: Point3,
    pub look_at: Point3,
    pub up_vec: Vector3,
    pub fov: f64,
    pub aperture: f64,
}

impl Default for CameraSettings {
    fn default() -> Self {
        CameraSettings {
            look_from: point3(0.0, 0.0, 0.0),
            look_at: point3(0.0, 0.0, -1.0),
            up_vec: vec3(0.0, 1.0, 0.0),
            fov: 45.0,
            aperture: 1.0 / 16.0,
        }
    }
}

impl CameraSettings {
    pub fn build_with_aspect_ratio(&self, aspect_ratio: f64) -> Camera {
        let theta = self.fov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focus_dist = (self.look_from - self.look_at).magnitude();

        let _w = (self.look_from - self.look_at).normalize();
        let u = self.up_vec.cross(_w);
        let v = _w.cross(u);

        let origin = self.look_from;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - _w * focus_dist;

        let lens_radius = self.aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            _w,
            u,
            v,
            lens_radius,
            settings: self.clone(),
        }
    }

    pub fn build_with_dimensions(&self, width: u32, height: u32) -> Camera {
        self.build_with_aspect_ratio(width as f64 / height as f64)
    }
}

impl Camera {
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
