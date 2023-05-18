use std::mem::take;

use cgmath::vec3;
use serde::{Serialize, Deserialize};

use crate::{
    render::{material::Material, Point3, Ray},
    vec_repo::Id,
};

use super::{HitRecord, HittableTrait};

#[derive(Clone, Serialize, Deserialize)]
pub struct Rect {
    min_point: Point3,
    max_point: Point3,
    material_id: Id<Material>,
}

impl Rect {
    pub fn new(min_point: Point3, max_point: Point3, material_id: Id<Material>) -> Self {
        Self {
            min_point,
            max_point,
            material_id,
        }
    }
}

impl HittableTrait for Rect {
    fn get_position(&self) -> Point3 {
        self.min_point
    }

    fn set_position(&mut self, c: Point3) {
        self.max_point += c - self.min_point;
        self.min_point = c;
    }

    fn hit_bounded(&self, ray: &Ray, min_dist: f64, max_dist: f64) -> Option<HitRecord> {
        let (x0, y0, z0) = self.min_point.into();
        let (x1, y1, z1) = self.max_point.into();
        if x0 == x1 {
            return hit_plane_yz(
                ray,
                [y0, y1, z0, z1, x0],
                self.material_id,
                min_dist,
                max_dist,
            );
        }
        if y0 == y1 {
            return hit_plane_xz(
                ray,
                [x0, x1, z0, z1, y0],
                self.material_id,
                min_dist,
                max_dist,
            );
        }
        if z0 == z1 {
            return hit_plane_xy(
                ray,
                [x0, x1, y0, y1, z0],
                self.material_id,
                min_dist,
                max_dist,
            );
        }
        let side1 = hit_plane_xy(
            ray,
            [x0, x1, y0, y1, z0],
            self.material_id,
            min_dist,
            max_dist,
        );
        let side2 = hit_plane_xy(
            ray,
            [x0, x1, y0, y1, z1],
            self.material_id,
            min_dist,
            max_dist,
        );
        let side3 = hit_plane_xz(
            ray,
            [x0, x1, z0, z1, y0],
            self.material_id,
            min_dist,
            max_dist,
        );
        let side4 = hit_plane_xz(
            ray,
            [x0, x1, z0, z1, y1],
            self.material_id,
            min_dist,
            max_dist,
        );
        let side5 = hit_plane_yz(
            ray,
            [y0, y1, z0, z1, x0],
            self.material_id,
            min_dist,
            max_dist,
        );
        let side6 = hit_plane_yz(
            ray,
            [y0, y1, z0, z1, x1],
            self.material_id,
            min_dist,
            max_dist,
        );
        let mut sides = [side1, side2, side3, side4, side5, side6];
        sides.sort_by(|lhs, rhs| {
            if rhs.is_none() {
                return std::cmp::Ordering::Less;
            }
            if lhs.is_none() {
                return std::cmp::Ordering::Greater;
            }
            lhs.as_ref().unwrap().distance.total_cmp(&rhs.as_ref().unwrap().distance)
        });
        take(&mut sides[0])
    }

    fn name(&self) -> &'static str {
        "Rect"
    }
}

fn hit_plane_xy(
    ray: &Ray,
    plane: [f64; 5],
    mat: Id<Material>,
    min_dist: f64,
    max_dist: f64,
) -> Option<HitRecord> {
    let [x0, x1, y0, y1, z] = plane;
    let dist = (z - ray.origin.z) / ray.direction.z;
    if dist > max_dist || dist < min_dist {
        return None;
    }
    let point = ray.at(dist);
    if point.x < x0 || point.x > x1 || point.y < y0 || point.y > y1 {
        return None;
    }
    let u = (point.x - x0) / (x1 - x0);
    let v = (point.y - y0) / (y1 - y0);
    Some(HitRecord::new(ray, dist, vec3(0.0, 0.0, 1.0), (u, v), mat))
}

fn hit_plane_xz(
    ray: &Ray,
    plane: [f64; 5],
    mat: Id<Material>,
    min_dist: f64,
    max_dist: f64,
) -> Option<HitRecord> {
    let [x0, x1, z0, z1, y] = plane;
    let dist = (y - ray.origin.y) / ray.direction.y;
    if dist > max_dist || dist < min_dist {
        return None;
    }
    let point = ray.at(dist);
    if point.x < x0 || point.x > x1 || point.z < z0 || point.z > z1 {
        return None;
    }
    let u = (point.x - x0) / (x1 - x0);
    let v = (point.z - z0) / (z1 - z0);
    Some(HitRecord::new(ray, dist, vec3(0.0, 1.0, 0.0), (u, v), mat))
}

fn hit_plane_yz(
    ray: &Ray,
    plane: [f64; 5],
    mat: Id<Material>,
    min_dist: f64,
    max_dist: f64,
) -> Option<HitRecord> {
    let [y0, y1, z0, z1, x] = plane;
    let dist = (x - ray.origin.x) / ray.direction.x;
    if dist > max_dist || dist < min_dist {
        return None;
    }
    let point = ray.at(dist);
    if point.y < y0 || point.y > y1 || point.z < z0 || point.z > z1 {
        return None;
    }
    let u = (point.y - y0) / (y1 - y0);
    let v = (point.z - z0) / (z1 - z0);
    Some(HitRecord::new(ray, dist, vec3(1.0, 0.0, 0.0), (u, v), mat))
}
