use nalgebra::{vector, Vector3};
use crate::{Float, Integrator, Randomness, Scene};
use crate::ray::Ray;
use crate::world::material::ScatteredRay;

pub struct PathTracingIntegrator {
    max_depth: u32,
    background_color: Vector3<Float>,
}
impl PathTracingIntegrator {
    pub fn new(depth: u32, background_color: Vector3<Float>) -> Self {
        Self {
            max_depth: depth,
            background_color,
        }
    }
}
impl Integrator for PathTracingIntegrator {
    fn cast_ray<R: Randomness>(&self, ray: &Ray, t_min: Float, t_max: Float, scene: &Scene, depth: u32, rng: &mut R) -> Vector3<Float> {
        if depth >= self.max_depth {
            return vector!(0.0, 0.0, 0.0);
        }

        if let Some(int) = scene.intersect(ray, t_min, t_max) {
            let mat = int.material;
            let emitted = scene.world.emit(mat, -ray.direction, &int);

            if let Some(ScatteredRay {
                            ray_out,
                            albedo,
                            pdf,
                        }) = scene.world.scatter_ray(mat, ray.direction, &int, rng) {

                let brdf = scene.world.brdf(mat, ray_out.direction, &int, -ray.direction);
                let cos_theta = int.normal.dot(&ray_out.direction).abs();
                let recursed = self.cast_ray(&ray_out, t_min, t_max, scene, depth + 1, rng);


                emitted + (mul_vectors(&albedo, &recursed) * brdf * cos_theta) / pdf
            }
            else {
                emitted
            }
        }
        else {
            self.background_color
        }
    }
}


fn mul_vectors(a: &Vector3<Float>, b: &Vector3<Float>) -> Vector3<Float> {
    Vector3::new(
        a[0] * b[0],
        a[1] * b[1],
        a[2] * b[2],
    )
}
