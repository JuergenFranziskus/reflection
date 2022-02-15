use nalgebra::{Point3, Unit, UnitQuaternion, UnitVector3, vector, Vector3};
use num_traits::FloatConst;
use crate::aabb::AABB;
use crate::{Float, Randomness, Scene};
use crate::intersection::Intersection;
use crate::pdf::PDF;
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
    pub fn intersects(&self, ray: &Ray, t_min: Float, t_max: Float) -> bool {
        // TODO: find a better way to do this that is super fast and shit
        self.intersect(ray, t_min, t_max).is_some()
    }

    pub fn area(&self) -> Float {
        match self {
            Self::Sphere { radius, .. } => 4.0 * Float::PI() * radius.powi(2),
        }
    }
    pub fn solid_angle(&self, o: Point3<Float>) -> Float {
        match self {
            Self::Sphere { origin, radius, .. } => {
                let cos_theta_max = (1.0 - radius.powi(2) / (origin - o).magnitude_squared()).sqrt();
                let solid_angle = 2.0 * Float::PI() * (1.0 - cos_theta_max);

                solid_angle
            }
        }
    }

    pub fn random_point_on_surface(&self, rng: &mut dyn Randomness) -> Point3<Float> {
        match self {
            Self::Sphere { origin, radius, .. } => {
                let dir = rng.unit_vector();
                let actual_radius = *radius;
                let point = origin + dir.into_inner() * actual_radius;
                point
            }
        }
    }
    pub fn random_direction_towards(&self, o: Point3<Float>, rng: &mut dyn Randomness) -> UnitVector3<Float> {
        match self {
            Self::Sphere { origin, radius, .. } => {
                let dir = rng.unit_vector();
                // move point slightly into the sphere to that the intersection works when calculating pdf value.
                let actual_radius = *radius - 0.01;
                let point = origin + dir.into_inner() * actual_radius;

                Unit::new_normalize(point - o)
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


pub struct PrimitiveDirectionPDF {
    o: Point3<Float>,
    primitive: PrimitiveRef,
}
impl PrimitiveDirectionPDF {
    pub fn new(o: Point3<Float>, primitive: PrimitiveRef) -> Self {
        Self {
            o,
            primitive,
        }
    }
}
impl PDF<UnitVector3<Float>> for PrimitiveDirectionPDF {
    fn value(&self, direction: &UnitVector3<Float>, scene: &Scene) -> Float {
        let p = &scene.primitives[self.primitive.0].primitive;

        if !p.intersects(&Ray::new(self.o, *direction), 0.001, Float::INFINITY) {
            return 0.0
        }

        let solid_angle = p.solid_angle(self.o);
        1.0 / solid_angle

    }
    fn generate(&self, rng: &mut dyn Randomness, scene: &Scene) -> UnitVector3<Float> {
        scene.primitives[self.primitive.0].primitive.random_direction_towards(self.o, &mut *rng)
    }
}


pub struct PrimitiveSurfacePDF {
    primitive: PrimitiveRef,
}
impl PrimitiveSurfacePDF {
    pub fn new(primitive: PrimitiveRef) -> Self {
        Self {
            primitive,
        }
    }
}
impl PDF<Point3<Float>> for PrimitiveSurfacePDF {
    fn value(&self, _value: &Point3<Float>, scene: &Scene) -> Float {
        let p = &scene.primitives[self.primitive.0].primitive;
        let area = p.area();
        1.0 / area
    }

    fn generate(&self, rng: &mut dyn Randomness, scene: &Scene) -> Point3<Float> {
        let p = &scene.primitives[self.primitive.0].primitive;
        p.random_point_on_surface(rng)
    }
}
