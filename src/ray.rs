use nalgebra::{Point3, Unit, Vector3};
use crate::Float;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point3<Float>,
    pub direction: Unit<Vector3<Float>>,
}
impl Ray {
    pub fn new(origin: Point3<Float>, direction: Unit<Vector3<Float>>) -> Self {
        Self {
            origin,
            direction
        }
    }

    pub fn point_at(&self, t: Float) -> Point3<Float> {
        self.origin + self.direction.into_inner() * t
    }
}
