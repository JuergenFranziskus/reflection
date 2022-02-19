use generational_arena::Index;
use nalgebra::Vector3;
use crate::{Float, Texture2D};
use crate::texture::TextureCoord2D;

pub enum Albedo {
    SolidColor(Vector3<Float>),
    Texture(Texture2D<Vector3<Float>>),
}
impl Albedo {
    pub fn sample(&self, coord: &TextureCoord2D) -> Vector3<Float> {
        match self {
            Self::SolidColor(c) => *c,
            Self::Texture(t) => t[*coord],
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AlbedoRef(pub(crate) Index);

