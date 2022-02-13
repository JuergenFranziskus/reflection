use nalgebra::{Unit, Vector3};
use crate::Float;

pub trait Randomness {
    fn float(&mut self) -> Float;

    fn usize_range_exclusive(&mut self, min: usize, max: usize) -> usize;
    
    fn unit_vector(&mut self) -> Unit<Vector3<Float>>;
}
pub trait SeedingRandomness {
    fn seed_new(&mut self) -> Self;
}
