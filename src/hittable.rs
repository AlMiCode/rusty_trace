use crate::material::Material;
use crate::repo::Id;
use crate::{Point3, Ray, Vector3};
use cgmath::{vec3, EuclideanSpace, InnerSpace};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vector3,
    pub distance: f64,
    pub uv: (f64, f64),
    pub front_face: bool,
    pub material_id: Id<dyn Material>,
}

impl HitRecord {
    fn new(
        ray: &Ray,
        distance: f64,
        outward_normal: Vector3,
        uv: (f64, f64),
        material_id: Id<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            point: ray.at(distance),
            normal,
            distance,
            uv,
            front_face,
            material_id,
        }
    }
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + HittableClone {
    fn name(&self) -> &'static str;
    fn hit_bounded(&self, ray: &Ray, min_dist: f64, max_dist: f64) -> Option<HitRecord>;
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        self.hit_bounded(ray, f64::EPSILON, f64::INFINITY)
    }
    fn get_position(&self) -> Point3;
    fn set_position(&mut self, c: Point3);
}

pub trait HittableClone {
    fn clone_box(&self) -> Box<dyn Hittable>;
}

impl<T> HittableClone for T
where
    T: 'static + Hittable + Clone,
{
    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Box<dyn Hittable> {
        self.clone_box()
    }
}

pub type HittableVec = Vec<Box<dyn Hittable>>;
impl Hittable for HittableVec {
    fn hit_bounded(&self, ray: &Ray, min_dist: f64, max_dist: f64) -> Option<HitRecord> {
        let mut result = None;
        let mut closest_dist = f64::INFINITY;
        for shape in self {
            let hit = shape.hit_bounded(ray, min_dist, max_dist);
            if let Some(hit) = hit {
                if hit.distance < closest_dist {
                    closest_dist = hit.distance;
                    result = Some(hit);
                }
            }
        }
        result
    }

    fn get_position(&self) -> Point3 {
        unimplemented!()
    }
    fn set_position(&mut self, _c: Point3) {
        unimplemented!()
    }

    fn name(&self) -> &'static str {
        "Group"
    }
}

#[derive(Clone)]
struct Translate {
    offset: Vector3,
    object: Box<dyn Hittable>,
}

impl Translate {
    fn new(object: Box<dyn Hittable>, offset: Vector3) -> Self {
        Self { object, offset }
    }
}

impl Hittable for Translate {
    fn hit_bounded(&self, ray: &Ray, min_dist: f64, max_dist: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction);
        self.object
            .hit_bounded(&moved_ray, min_dist, max_dist)
            .and_then(|mut hit| {
                hit.point += self.offset;
                hit.set_face_normal(ray, hit.normal);
                Some(hit)
            })
    }

    fn set_position(&mut self, c: Point3) {
        self.offset = c.to_vec();
    }

    fn get_position(&self) -> Point3 {
        Point3::from_vec(self.offset)
    }

    fn name(&self) -> &'static str {
        "Translate"
    }
}

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material_id: Id<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material_id: Id<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material_id,
        }
    }

    pub fn get_uv(normal: &Vector3) -> (f64, f64) {
        let pi = std::f64::consts::PI;
        let phi = (-normal.z).atan2(normal.x) + pi;
        let u = phi / (2.0 * pi);
        let v = normal.y.asin() / pi + 0.5;
        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit_bounded(&self, ray: &Ray, min_dist: f64, max_dist: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let half_b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;
        if root < min_dist || root > max_dist {
            root = (-half_b + sqrt_d) / a;
            if root < min_dist || root > max_dist {
                return None;
            }
        }
        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;
        Some(HitRecord::new(
            ray,
            root,
            outward_normal,
            Sphere::get_uv(&outward_normal),
            self.material_id,
        ))
    }

    fn get_position(&self) -> Point3 {
        self.center
    }
    fn set_position(&mut self, c: Point3) {
        self.center = c;
    }

    fn name(&self) -> &'static str {
        "Sphere"
    }
}

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

    material_id: Id<dyn Material>,
}

impl Rect {
    pub fn new(p0: &Point3, n: f64, m: f64, plane: Plane, material_id: Id<dyn Material>) -> Self {
        match plane {
            Plane::XY => Self { n0: p0.x, m0: p0.y, n1: p0.x + n, m1: p0.y + m, k: p0.z, plane, material_id },
            Plane::XZ => Self { n0: p0.x, m0: p0.z, n1: p0.x + n, m1: p0.z + m, k: p0.y, plane, material_id },
            Plane::YZ => Self { n0: p0.y, m0: p0.z, n1: p0.y + n, m1: p0.z + m, k: p0.x, plane, material_id }
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
