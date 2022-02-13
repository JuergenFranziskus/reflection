use nalgebra::Vector3;
use crate::{Float, Integrator, Randomness, Scene};
use crate::ray::Ray;

pub struct NormalIntegrator;
impl Integrator for NormalIntegrator {
    fn cast_ray<R: Randomness>(&self, ray: &Ray, t_min: Float, t_max: Float, scene: &Scene, _depth: u32, _rng: &mut R) -> Vector3<Float> {
        if let Some(int) = scene.intersect(ray, t_min, t_max) {
            int.normal.into_inner()
        }
        else {
            Vector3::zeros()
        }
    }
}
