use generational_arena::Index;
use nalgebra::{Isometry3, Point3};
use crate::Float;
use crate::scene::primitive::Primitive;


pub enum Shape {
    Sphere {
        radius: Float,
    }
}
impl Shape {
    pub fn as_transformed_primitives(&self, t: &Isometry3<Float>) -> Vec<Primitive> {
        match self {
            Self::Sphere { radius } => {
                let origin: Point3<Float> = t.translation.vector.into();

                vec!(Primitive::Sphere {
                    origin,
                    rotation: t.rotation,
                    radius: *radius,
                })
            }
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShapeRef(pub(crate) Index);
