use generational_arena::Index;
use nalgebra::Vector3;
use crate::Float;
use crate::intersection::Intersection;

pub enum Albedo {
    SolidColor(Vector3<Float>),
}
impl Albedo {
    pub fn sample(&self, _int: &Intersection) -> Vector3<Float> {
        match self {
            Self::SolidColor(c) => *c,
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AlbedoRef(pub(crate) Index);

