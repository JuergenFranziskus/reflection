use generational_arena::Index;
use nalgebra::{Unit, UnitVector3, Vector3};
use num_traits::FloatConst;
use crate::{Float, Randomness};
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::world::albedo::AlbedoRef;
use crate::world::World;
use num_traits::identities::Zero;


pub enum Material {
    Lambertian(AlbedoRef),
}
impl Material {
    pub fn scatter<R: Randomness>(&self, _ray_in: UnitVector3<Float>, int: &Intersection, world: &World, rng: &mut R) -> Option<ScatteredRay> {
        match self {
            Self::Lambertian(albedo) => {
                let mut scatter_direction = int.normal.into_inner() + rng.unit_vector().into_inner();
                if scatter_direction.is_zero() {
                    scatter_direction = int.normal.into_inner();
                }

                let scattered = Ray::new(int.point, Unit::new_normalize(scatter_direction));
                let albedo = world.sample_albedo(*albedo, int);
                let pdf = int.normal.dot(&scattered.direction) / Float::PI();


                Some(ScatteredRay {
                    ray_out: scattered,
                    albedo,
                    pdf,
                })
            }
        }
    }

    pub fn brdf(&self, ray_in: UnitVector3<Float>, int: &Intersection, _ray_out: UnitVector3<Float>) -> Float {
        match self {
            Self::Lambertian(_) => {
                let cosine = int.normal.dot(&ray_in);

                if cosine < 0.0 {
                    0.0
                }
                else {
                    1.0 / Float::PI()
                }
            }
        }
    }
    
    pub fn emit(&self, _ray_out: UnitVector3<Float>, _int: &Intersection, _world: &World) -> Vector3<Float> {
        match self {
            Self::Lambertian(_) => Vector3::zeros(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MaterialRef(pub(crate) Index);


pub struct ScatteredRay {
    pub ray_out: Ray,
    pub albedo: Vector3<Float>,
    pub pdf: Float,
}
