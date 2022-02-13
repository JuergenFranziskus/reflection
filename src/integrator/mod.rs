use nalgebra::Vector3;
use crate::camera::Camera;
use crate::{Float};
use crate::randomness::Randomness;
use crate::ray::Ray;
use crate::scene::Scene;

pub mod normal_integrator;
pub mod path_integrator;

pub trait Integrator {
    fn cast_ray<R: Randomness>(&self, ray: &Ray, t_min: Float, t_max: Float, scene: &Scene, depth: u32, rng: &mut R) -> Vector3<Float>;
    fn render_line<R: Randomness>(
        &self,
        line: &mut [Vector3<Float>],
        camera: &Camera,
        y: u32,
        width: u32,
        height: u32,
        samples: u32,
        scene: &Scene,
        t_min: Float,
        t_max: Float,
        rng: &mut R,
    ) {
        let t_width = width as Float;
        let t_height = height as Float;
        let factor = 1.0 / samples as Float;

        for x in 0..width {
            let mut color = Vector3::zeros();

            for _ in 0..samples {
                let x_offset: Float = rng.float();
                let y_offset: Float = rng.float();

                let x_coord = x as Float + x_offset;
                let y_coord = y as Float + y_offset;
                let ray = camera.get_ray(x_coord / t_width, y_coord / t_height);

                let ray_color = self.cast_ray(&ray, t_min, t_max, scene, 0, rng);
                let sample = ray_color * factor;
                color += sample;
            }


            line[x as usize] = color;
        }
    }
}
