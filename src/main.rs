#![allow(dead_code)]

use std::time::Instant;
use image::ImageFormat;
use nalgebra::{Isometry3, Point3, Unit, vector, Vector3};
use reflection::camera::Camera;
use reflection::{Float, render, RenderDescriptor};
use rand::prelude::*;
use rand_distr::UnitSphere;
use reflection::integrator::path_integrator::PathTracingIntegrator;
use reflection::randomness::{Randomness, SeedingRandomness};
use reflection::world::material::MaterialRef;
use reflection::world::World;


const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const SAMPLES: u32 = 16;
const DEPTH: u32 = 4;
const ASPECT: Float = WIDTH as Float / HEIGHT as Float;

const RNG_SEED: u64 = 100;
const GAMMA: Float = 1.0;

fn main() {
    let rng = StdRng::seed_from_u64(RNG_SEED);

    let mut randomness = DefaultRandomness { rng };

    let (world, camera) = build_world(&mut randomness);

    let build_start = Instant::now();
    let scene = world.build_scene(&mut randomness);
    let build_took = build_start.elapsed();

    let integrator = PathTracingIntegrator::new(DEPTH, vector!(1.0, 1.0, 1.0));

    let start = Instant::now();
    let render = render(RenderDescriptor {
        width: WIDTH,
        height: HEIGHT,
        samples: SAMPLES,
        t_min: 0.001,
        t_max: Float::INFINITY,
        integrator,
        rng: &mut randomness,
        scene,
        camera
    });
    let took = start.elapsed();

    let pixels: Vec<_> = render.into_pixels()
        .map(color_to_rgb)
        .flatten()
        .collect();
    let image = image::RgbImage::from_raw(WIDTH, HEIGHT, pixels).unwrap();
    image.save_with_format("image.png", ImageFormat::Png).unwrap();

    println!("Built scene in {} milliseconds", build_took.as_millis());
    println!("Finished render in {:.2} seconds", took.as_secs_f64());
}

fn color_to_rgb(mut c: Vector3<Float>) -> [u8; 3] {
    c = color_correct(c);

    let r = (c[0] * 255.0).clamp(0.0, 255.0) as u8;
    let g = (c[1] * 255.0).clamp(0.0, 255.0) as u8;
    let b = (c[2] * 255.0).clamp(0.0, 255.0) as u8;

    [r, g, b]
}
fn color_correct(mut c: Vector3<Float>) -> Vector3<Float> {
    c[0] = c[0].powf(1.0 / GAMMA);
    c[1] = c[1].powf(1.0 / GAMMA);
    c[2] = c[2].powf(1.0 / GAMMA);

    c
}
fn color_decorrect(mut c: Vector3<Float>) -> Vector3<Float> {
    c[0] = c[0].powf(GAMMA);
    c[1] = c[1].powf(GAMMA);
    c[2] = c[2].powf(GAMMA);

    c
}

fn build_world<R: Randomness>(rng: &mut R) -> (World, Camera) {
    let mut world = World::new();

    let ground_shape = world.add_sphere(1000.0);
    let ground_mat = random_lambertian(&mut world, rng);
    world.add_object(ground_shape, ground_mat, Isometry3::translation(0.0, -1000.0, 0.0));

    let small_sphere = world.add_sphere(0.2);
    
    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as Float + 0.9 * rng.float(),
                0.2,
                b as Float + 0.9 * rng.float(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let mat = random_lambertian(&mut world, rng);
                world.add_object(small_sphere, mat, Isometry3::translation(center.x, center.y, center.z));
            }
        }
    }

    let sphere = world.add_sphere(1.0);

    let mat0 = random_lambertian(&mut world, rng);
    let mat1 = random_lambertian(&mut world, rng);
    let mat2 = random_lambertian(&mut world, rng);

    world.add_object(sphere, mat0, Isometry3::translation( 0.0, 1.0, 0.0));
    world.add_object(sphere, mat1, Isometry3::translation(-4.0, 1.0, 0.0));
    world.add_object(sphere, mat2, Isometry3::translation( 4.0, 1.0, 0.0));


    let camera = Camera::new(
        Point3::new(13.0, 2.0, 3.0) * 2.0,
        Point3::origin(),
        Vector3::new(0.0, 1.0, 0.0),
        (20.0 as Float).to_radians(),
        ASPECT,
    );

    (world, camera)
}
fn random_lambertian<R: Randomness>(world: &mut World, rng: &mut R) -> MaterialRef {
    let albedo = random_albedo(rng);
    let a_ref = world.add_solid_albedo(albedo);
    world.add_lambertian_material(a_ref)
}
fn random_albedo<R: Randomness>(rng: &mut R) -> Vector3<Float> {
    let r = rng.float();
    let g = rng.float();
    let b = rng.float();

    Vector3::new(r, g, b)
}

pub struct DefaultRandomness {
    pub rng: StdRng,
}
impl Randomness for DefaultRandomness {
    fn float(&mut self) -> Float {
        self.rng.gen()
    }
    fn usize_range_exclusive(&mut self, min: usize, max: usize) -> usize {
        self.rng.gen_range(min..max)
    }

    fn unit_vector(&mut self) -> Unit<Vector3<Float>> {
        let [x, y, z] = UnitSphere.sample(&mut self.rng);
        Unit::new_unchecked(vector!(x, y, z))
    }
}
impl SeedingRandomness for DefaultRandomness {
    fn seed_new(&mut self) -> Self {
        Self {
            rng: StdRng::seed_from_u64(self.rng.gen())
        }
    }
}
