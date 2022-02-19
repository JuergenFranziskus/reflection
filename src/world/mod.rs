use generational_arena::{Arena, Index};
use nalgebra::{Isometry3, UnitVector3, Vector3};
use crate::{Float, Texture2D};
use crate::intersection::Intersection;
use crate::randomness::Randomness;
use crate::scene::Scene;
use crate::texture::TextureCoord2D;
use crate::world::albedo::{Albedo, AlbedoRef};
use crate::world::material::{Material, MaterialRef, ScatteredRay};
use crate::world::shape::{Shape, ShapeRef};

pub mod shape;
pub mod albedo;
pub mod material;

pub struct World {
    pub(crate) shapes: Arena<Shape>,
    pub(crate) albedos: Arena<Albedo>,
    pub(crate) materials: Arena<Material>,
    pub(crate) objects: Arena<Object>,
}
impl World {
    pub fn new() -> World {
        World {
            shapes: Arena::new(),
            albedos: Arena::new(),
            materials: Arena::new(),
            objects: Arena::new(),
        }
    }

    pub fn add_sphere(&mut self, radius: Float) -> ShapeRef {
        let i = self.shapes.insert(Shape::Sphere { radius });
        ShapeRef(i)
    }
    pub fn add_solid_albedo(&mut self, albedo: Vector3<Float>) -> AlbedoRef {
        let i = self.albedos.insert(Albedo::SolidColor(albedo));
        AlbedoRef(i)
    }
    pub fn add_texture_albedo(&mut self, texture: Texture2D<Vector3<Float>>) -> AlbedoRef {
        let i = self.albedos.insert(Albedo::Texture(texture));
        AlbedoRef(i)
    }
    pub fn add_lambertian_material(&mut self, albedo: AlbedoRef) -> MaterialRef {
        let i = self.materials.insert(Material::Lambertian(albedo));
        MaterialRef(i)
    }
    pub fn add_mirror_material(&mut self) -> MaterialRef {
        let i = self.materials.insert(Material::Mirror);
        MaterialRef(i)
    }
    pub fn add_emitting_material(&mut self, albedo: AlbedoRef, factor: Float) -> MaterialRef {
        let i = self.materials.insert(Material::Emitting(albedo, factor));
        MaterialRef(i)
    }
    pub fn add_object(&mut self, shape: ShapeRef, mat: MaterialRef, transform: Isometry3<Float>) -> ObjectRef {
        let i = self.objects.insert(Object { shape, material: mat, transform });
        ObjectRef(i)
    }


    pub fn sample_albedo(&self, albedo: AlbedoRef, coord: &TextureCoord2D) -> Vector3<Float> {
        let a = &self.albedos[albedo.0];
        a.sample(coord)
    }
    pub fn scatter_ray(&self, mat: MaterialRef, ray_in: UnitVector3<Float>, int: &Intersection, scene: &Scene) -> Option<ScatteredRay> {
        let m = &self.materials[mat.0];
        m.scatter(ray_in, int, scene)
    }
    pub fn brdf(&self, mat: MaterialRef, ray_in: UnitVector3<Float>, int: &Intersection, ray_out: UnitVector3<Float>) -> Float {
        let m = &self.materials[mat.0];
        m.brdf(ray_in, int, ray_out)
    }
    pub fn emit(&self, mat: MaterialRef, ray_out: UnitVector3<Float>, int: &Intersection) -> Vector3<Float> {
        let m = &self.materials[mat.0];
        m.emit(ray_out, int, self)
    }
    pub fn emits(&self, mat: MaterialRef) -> bool {
        self.materials[mat.0].emits()
    }

    pub fn build_scene<R: Randomness>(&self, rng: &mut R) -> Scene {
        Scene::new(self, rng)
    }
}



pub struct Object {
    pub(crate) shape: ShapeRef,
    pub(crate) material: MaterialRef,
    pub(crate) transform: Isometry3<Float>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ObjectRef(pub(crate) Index);
