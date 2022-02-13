use crate::Float;
use crate::intersection::Intersection;
use crate::randomness::Randomness;
use crate::ray::Ray;
use crate::scene::bvh::BVH;
use crate::scene::primitive::{Primitive, PrimitiveRef};
use crate::world::material::MaterialRef;
use crate::world::World;

pub mod primitive;
pub mod bvh;


pub struct Scene<'a> {
    pub(crate) world: &'a World,
    pub(crate) primitives: Vec<PrimitiveData>,
    pub(crate) bvh: BVH,
}
impl<'a> Scene<'a> {
    pub fn new<R: Randomness>(world: &'a World, rng: &mut R) -> Scene<'a> {
        let mut primitives = Vec::new();
        let mut aabbs = Vec::new();

        for (object_id, (_, o)) in world.objects.iter().enumerate() {
            let shape = &world.shapes[o.shape.0];
            let t = &o.transform;
            let material = o.material;

            let t_primitives = shape.as_transformed_primitives(t);

            for p in t_primitives {
                let primitive_i = primitives.len();
                let aabb = p.aabb();

                aabbs.push((PrimitiveRef(primitive_i), aabb));
                primitives.push(PrimitiveData {
                    primitive: p,
                    material,
                    _object_id: object_id,
                });
            }
        }

        let bvh = BVH::new(&mut aabbs, rng);

        Scene {
            world,
            primitives,
            bvh,
        }
    }

    pub fn intersect(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<Intersection> {
        let find = |ray: &Ray, p: PrimitiveRef| {
            let mat = self.primitives[p.0].material;
            self.primitives[p.0].primitive.intersect(ray, t_min, t_max).map(|i| i.to_intersection(mat))
        };
        let comp = |a: Intersection, b: Intersection| {
            if a.t < b.t {
                a
            }
            else {
                b
            }
        };

        self.bvh.find_intersection(ray, find, comp, t_min, t_max)
    }
}


pub(crate) struct PrimitiveData {
    primitive: Primitive,
    material: MaterialRef,
    _object_id: usize,
}
