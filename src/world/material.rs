use generational_arena::Index;
use nalgebra::{Reflection3, Unit, UnitVector3, vector, Vector3};
use num_traits::FloatConst;
use crate::{Float, Randomness, Scene};
use crate::intersection::Intersection;
use crate::world::albedo::AlbedoRef;
use crate::world::World;
use num_traits::identities::Zero;
use crate::pdf::PDF;
use crate::ray::Ray;


pub enum Material {
    Lambertian(AlbedoRef),
    Mirror,
    Emitting(AlbedoRef, Float),
}
impl Material {
    pub fn scatter(&self, ray_in: UnitVector3<Float>, int: &Intersection, scene: &Scene) -> Option<ScatteredRay> {
        match self {
            Self::Lambertian(albedo) => {
                let pdf = MaterialPDF::Lambertian(CosinePDF::new(int.normal));
                let albedo = scene.world.sample_albedo(*albedo, &int.tex_coord);

                Some(ScatteredRay::Diffuse {
                    pdf,
                    albedo,
                })
            }
            Self::Mirror => {
                let reflection = Reflection3::new(int.normal, 0.0);
                let mut dir = -ray_in.into_inner();
                reflection.reflect(&mut dir);

                let ray = Ray::new(int.point, Unit::new_normalize(dir));

                Some(ScatteredRay::Specular {
                    ray,
                    albedo: vector!(1.0, 1.0, 1.0),
                })
            }
            Self::Emitting(_, _) => None,
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
            Self::Mirror => 1.0,
            Self::Emitting(_, _) => 0.0,
        }
    }
    
    pub fn emit(&self, _ray_out: UnitVector3<Float>, int: &Intersection, world: &World) -> Vector3<Float> {
        match self {
            Self::Lambertian(_) => Vector3::zeros(),
            Self::Mirror => Vector3::zeros(),
            Self::Emitting(a, factor) => world.sample_albedo(*a, &int.tex_coord) * *factor,
        }
    }
    
    pub fn emits(&self) -> bool {
        match self {
            Self::Lambertian(_) => false,
            Self::Mirror => false,
            Self::Emitting(_, _) => true,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MaterialRef(pub(crate) Index);


pub enum ScatteredRay {
    Diffuse {
        pdf: MaterialPDF,
        albedo: Vector3<Float>,
    },
    Specular {
        ray: Ray,
        albedo: Vector3<Float>,
    },
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
