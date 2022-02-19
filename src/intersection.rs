use nalgebra::{Point3, Unit, Vector3};
use crate::Float;
use crate::texture::TextureCoord2D;
use crate::world::material::MaterialRef;


pub struct Intersection {
    pub t: Float,
    pub point: Point3<Float>,
    pub normal: Unit<Vector3<Float>>,
    pub outside: bool,
    pub material: MaterialRef,
    pub tex_coord: TextureCoord2D,
}
