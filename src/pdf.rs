use crate::{Float, Randomness, Scene};



pub trait PDF<T>: Send + Sync {
    fn value(&self, value: &T, scene: &Scene) -> Float;
    fn generate(&self, rng: &mut dyn Randomness, scene: &Scene) -> T;
}
