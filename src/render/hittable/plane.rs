use cgmath::vec3;

use crate::render::{material::Material, repo::Id, Point3, Ray};

use super::{HitRecord, Hittable};

#[derive(Clone)]
pub enum Plane {
    XY,
    XZ,
    YZ,
}

#[derive(Clone)]
pub struct Rect {
    n0: f64,
    n1: f64,
    m0: f64,
    m1: f64,
    k: f64,
    plane: Plane,

    material_id: Id<Material>,
}

impl Rect {
    pub fn new(p0: &Point3, n: f64, m: f64, plane: Plane, material_id: Id<Material>) -> Self {
        match plane {
            Plane::XY => Self {
                n0: p0.x,
                m0: p0.y,
                n1: p0.x + n,
                m1: p0.y + m,
                k: p0.z,
                plane,
                material_id,
            },
            Plane::XZ => Self {
                n0: p0.x,
                m0: p0.z,
                n1: p0.x + n,
                m1: p0.z + m,
                k: p0.y,
                plane,
                material_id,
            },
            Plane::YZ => Self {
                n0: p0.y,
                m0: p0.z,
                n1: p0.y + n,
                m1: p0.z + m,
                k: p0.x,
                plane,
                material_id,
            },
        }
    }
}

impl Hittable for Rect {
    fn hit_bounded(&self, ray: &Ray, min_dist: f64, max_dist: f64) -> Option<HitRecord> {
        let (orig_k, dir_k) = match self.plane {
            Plane::XY => (ray.origin.z, ray.direction.z),
            Plane::XZ => (ray.origin.y, ray.direction.y),
            Plane::YZ => (ray.origin.x, ray.direction.x),
        };

        let dist = (self.k - orig_k) / dir_k;
        if dist > max_dist || dist < min_dist {
            return None;
        }
        let point = ray.at(dist);
        let (point_n, point_m, outward_normal) = match self.plane {
            Plane::XY => (point.x, point.y, vec3(0.0, 0.0, 1.0)),
            Plane::XZ => (point.x, point.z, vec3(0.0, 1.0, 0.0)),
            Plane::YZ => (point.y, point.z, vec3(1.0, 0.0, 0.0)),
        };
        if point_n < self.n0 || point_n > self.n1 || point_m < self.m0 || point_m > self.m1 {
            return None;
        }
        let u = (point_n - self.n0) / (self.n1 - self.n0);
        let v = (point_m - self.m0) / (self.m1 - self.m0);
        Some(HitRecord::new(
            ray,
            dist,
            outward_normal,
            (u, v),
            self.material_id,
        ))
    }

    fn get_position(&self) -> Point3 {
        match self.plane {
            Plane::XY => Point3::new(self.n0, self.m0, self.k),
            Plane::XZ => Point3::new(self.n0, self.k, self.m0),
            Plane::YZ => Point3::new(self.k, self.n0, self.m0),
        }
    }

    fn set_position(&mut self, _c: Point3) {
        unimplemented!("set_position and get_position is wrong way, as every object has its own properties. Will be replaced with add_modifier soon")
    }

    fn name(&self) -> &'static str {
        "Rectangle"
    }
}
