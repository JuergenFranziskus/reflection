use generational_arena::Index;
use nalgebra::{Unit, UnitVector3, Vector3};
use num_traits::FloatConst;
use crate::{Float, Randomness, Scene};
use crate::intersection::Intersection;
use crate::world::albedo::AlbedoRef;
use crate::world::World;
use num_traits::identities::Zero;
use crate::pdf::PDF;


pub enum Material {
    Lambertian(AlbedoRef),
    Emitting(AlbedoRef),
}
impl Material {
    pub fn scatter(&self, _ray_in: UnitVector3<Float>, int: &Intersection, scene: &Scene) -> Option<ScatteredRay> {
        match self {
            Self::Lambertian(albedo) => {
                let pdf = MaterialPDF::Lambertian(CosinePDF::new(int.normal));
                let albedo = scene.world.sample_albedo(*albedo, int);

                Some(ScatteredRay {
                    pdf,
                    albedo,
                })
            }
            Self::Emitting(_) => None,
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
            Self::Emitting(_) => 0.0,
        }
    }
    
    pub fn emit(&self, _ray_out: UnitVector3<Float>, int: &Intersection, world: &World) -> Vector3<Float> {
        match self {
            Self::Lambertian(_) => Vector3::zeros(),
            Self::Emitting(a) => world.sample_albedo(*a, int),
        }
    }
    
    pub fn emits(&self) -> bool {
        match self {
            Self::Lambertian(_) => false,
            Self::Emitting(_) => true,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MaterialRef(pub(crate) Index);


pub struct ScatteredRay {
    pub pdf: MaterialPDF,
    pub albedo: Vector3<Float>,
}

pub enum MaterialPDF {
    Lambertian(CosinePDF),
}
impl PDF<UnitVector3<Float>> for MaterialPDF {
    fn value(&self, direction: &UnitVector3<Float>, scene: &Scene) -> Float {
        match self {
            Self::Lambertian(c) => c.value(direction, scene),
        }
    }

    fn generate(&self, rng: &mut dyn Randomness, scene: &Scene) -> UnitVector3<Float> {
        match self {
            Self::Lambertian(c) => c.generate(rng, scene),
        }
    }
}


pub struct CosinePDF {
    normal: UnitVector3<Float>,
}
impl CosinePDF {
    pub fn new(normal: UnitVector3<Float>) -> Self {
        Self {
            normal,
        }
    }
}
impl PDF<UnitVector3<Float>> for CosinePDF {
    fn value(&self, direction: &UnitVector3<Float>, _scene: &Scene) -> Float {
        self.normal.dot(&direction) / Float::PI()
    }

    fn generate(&self, rng: &mut dyn Randomness, _scene: &Scene) -> UnitVector3<Float> {
        let mut scatter_direction = self.normal.into_inner() + rng.unit_vector().into_inner();
        if scatter_direction.is_zero() {
            scatter_direction = self.normal.into_inner();
        }

        Unit::new_normalize(scatter_direction)
    }
}
