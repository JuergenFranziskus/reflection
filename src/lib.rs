use nalgebra::Vector3;
use parking_lot::Mutex;
use crate::camera::Camera;
use crate::integrator::Integrator;
use crate::randomness::{Randomness, SeedingRandomness};
use crate::scene::Scene;
use rayon::prelude::*;
use crate::texture::Texture2D;

pub mod world;
pub mod scene;
pub mod aabb;
pub mod ray;
pub mod randomness;
pub mod intersection;
pub mod camera;
pub mod integrator;
pub mod texture;
pub mod pdf;

#[cfg(not(feature = "wide"))]
pub type Float = f32;
#[cfg(feature = "wide")]
pub type Float = f64;


pub fn render<I: Integrator + Sync, R: Randomness + SeedingRandomness + Send>(desc: RenderDescriptor<I, R>) -> Texture2D<Vector3<Float>> {
    let mut pixels = vec![Vector3::zeros(); desc.width as usize * desc.height as usize];

    let rng = Mutex::new(desc.rng);

    pixels.chunks_exact_mut(desc.width as usize).enumerate()
        .par_bridge()
        .for_each(|(i, p)| {
            let y = desc.height - i as u32 - 1;
            let mut rng = rng.lock().seed_new();

            desc.integrator.render_line(
                p,
                &desc.camera,
                y,
                desc.width,
                desc.height,
                desc.samples,
                &desc.scene,
                desc.t_min,
                desc.t_max,
                &mut rng
            );

            eprintln!("Finished line {}", y);
        });


    Texture2D::new_from_pixels(desc.width, desc.height, pixels)
}



pub struct RenderDescriptor<'a, I, R> {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    pub t_min: Float,
    pub t_max: Float,
    pub integrator: I,
    pub rng: &'a mut R,
    pub scene: Scene<'a>,
    pub camera: Camera,
}
