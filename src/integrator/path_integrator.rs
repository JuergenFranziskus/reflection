use nalgebra::{UnitVector3, vector, Vector3};
use crate::{Float, Integrator, Randomness, Scene};
use crate::intersection::Intersection;
use crate::pdf::PDF;
use crate::ray::Ray;
use crate::scene::primitive::{PrimitiveDirectionPDF, PrimitiveRef};
use crate::world::material::{ScatteredRay};

pub struct PathTracingIntegrator {
    max_depth: u32,
    background_color: Vector3<Float>,
    lights: Vec<PrimitiveRef>,
}
impl PathTracingIntegrator {
    pub fn new(depth: u32, background_color: Vector3<Float>, scene: &Scene) -> Self {
        let mut lights = Vec::new();

        for (i, p) in scene.primitives.iter().enumerate() {
            if scene.world.emits(scene.materials[p.object_id]) {
                lights.push(PrimitiveRef(i));
            }
        }

        Self {
            max_depth: depth,
            background_color,
            lights,
        }
    }

    fn bias_light<R: Randomness>(&self, int: &Intersection, scene: &Scene, rng: &mut R) -> Ray {
        let choice = rng.usize_range_exclusive(0, self.lights.len());
        let pdf = PrimitiveDirectionPDF::new(int.point, self.lights[choice]);
        let direction = pdf.generate(rng, scene);

        Ray::new(int.point, direction)
    }
    fn light_pdf_value(&self, value: UnitVector3<Float>, int: &Intersection, scene: &Scene) -> Float {
        let total_value: Float = self.lights.iter()
            .map(|l| PrimitiveDirectionPDF::new(int.point, *l).value(&value, scene))
            .sum();

        let dividend = self.lights.len() as Float;

        total_value / dividend
    }
    fn generate_new_ray<R: Randomness>(&self, scattered: &ScatteredRay, int: &Intersection, scene: &Scene, rng: &mut R) -> (Ray, Float) {
        if self.lights.len() != 0 && !scattered.is_specular {
            let choice = rng.usize_range_exclusive(0, 2);

            let ray = if choice == 0 {
                self.bias_light(int, scene, rng)
            } else {
                let dir = scattered.pdf.generate(rng, scene);
                Ray::new(int.point, dir)
            };

            let light_pdf_value = self.light_pdf_value(ray.direction, int, scene);
            let scattered_pdf_value = scattered.pdf.value(&ray.direction, scene);

            let total_pdf_value = 0.5 * scattered_pdf_value + 0.5 * light_pdf_value;
            (ray, total_pdf_value)
        } else {
            let ray = Ray::new(int.point, scattered.pdf.generate(rng, scene));
            let pdf = scattered.pdf.value(&ray.direction, scene);
            (ray, pdf)
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

            if let Some(scattered) = scene.world.scatter_ray(mat, -ray.direction, &int, scene) {
                let (ray_out, pdf) = self.generate_new_ray(&scattered, &int, scene, rng);

                let brdf = scene.world.brdf(mat, ray_out.direction, &int, -ray.direction);

                let mut recursed = self.cast_ray(&ray_out, t_min, t_max, scene, depth + 1, rng);
                correct_abnormal_color(&mut recursed);

                emitted + (mul_vectors(&scattered.attenuation, &recursed) * brdf) / pdf
            } else {
                emitted
            }
        } else {
            self.background_color
        }
    }
}

fn correct_abnormal_color(c: &mut Vector3<Float>)  {
    for x in 0..3 {
        if color_abnormal(c[x]) {
            c[x] = 0.0;
        }
    }


}
fn color_abnormal(c: Float) -> bool {
    c.is_nan() || c.is_infinite()
}
fn mul_vectors(a: &Vector3<Float>, b: &Vector3<Float>) -> Vector3<Float> {
    Vector3::new(
        a[0] * b[0],
        a[1] * b[1],
        a[2] * b[2],
    )
}
