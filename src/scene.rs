use cgmath::{Point3, Vector3};

mod shapes;

pub struct Scene {
    camera: Camera,
    shapes: Vec<Shape>
}
