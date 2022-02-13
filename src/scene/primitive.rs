use nalgebra::{Point3, Unit, UnitQuaternion, vector, Vector3};
use crate::aabb::AABB;
use crate::Float;
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::world::material::MaterialRef;

pub enum Primitive {
    Sphere {
        origin: Point3<Float>,
        rotation: UnitQuaternion<Float>,
        radius: Float,
    }
}
impl Primitive {
    pub fn aabb(&self) -> AABB {
        match self {
            Self::Sphere { origin, radius, .. } => {
                let diff = vector!(*radius, *radius, *radius);
                let min = origin - diff;
                let max = origin + diff;

                AABB::new(min, max)
            }
        }
    }

    pub fn intersect(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<PrimitiveIntersection> {
        match self {
            Self::Sphere { origin, rotation, radius } => {
                intersect_sphere(
                    *origin,
                    *rotation,
                    *radius,
                    ray,
                    t_min,
                    t_max,
                )
            }
        }
    }
}

fn intersect_sphere(
    origin: Point3<Float>,
    _rotation: UnitQuaternion<Float>,
    radius: Float,
    ray: &Ray,
    t_min: Float,
    t_max: Float
) -> Option<PrimitiveIntersection> {
    let oc = ray.origin - origin;
    let b = oc.dot(&ray.direction.into_inner());
    let c = oc.magnitude_squared() - radius.powi(2);
    let descrim = b * b - c;

    let t = if descrim > 0.0 {
        let desc_sqrt = descrim.sqrt();

        let t0 = -b - desc_sqrt;
        let t1 = -b + desc_sqrt;

        if t0 >= t_min && t0 <= t_max {
            Some((t0, true))
        }
        else if t1 >= t_min && t1 <= t_max {
            Some((t1, false))
        }
        else {
            None
        }
    } else {
        None
    };


    if let Some((t, outside)) = t {
        let point = ray.point_at(t);
        let normal = if outside {
            Unit::new_normalize(point - origin)
        } else {
            Unit::new_normalize(origin - point)
        };


        Some(PrimitiveIntersection {
            t,
            normal,
            point,
            outside,
        })
    }
    else {
        None
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrimitiveRef(pub(crate) usize);


pub struct PrimitiveIntersection {
    pub t: Float,
    pub point: Point3<Float>,
    pub normal: Unit<Vector3<Float>>,
    pub outside: bool,
}
impl PrimitiveIntersection {
    pub fn to_intersection(self, mat: MaterialRef) -> Intersection {
        Intersection {
            t: self.t,
            point: self.point,
            normal: self.normal,
            outside: self.outside,
            material: mat,
        }
    }
}
