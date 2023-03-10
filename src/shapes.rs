use crate::Point3;

// temporary shape = sphere
// in the future shape will be a tagged union(?)
pub struct Shape {
    pub center: Point3,
    pub radius: f64,
}
