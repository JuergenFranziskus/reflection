use std::mem::swap;
use nalgebra::Point3;
use crate::Float;
use crate::ray::Ray;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AABB {
    pub(crate) min: Point3<Float>,
    pub(crate) max: Point3<Float>,
}
impl AABB {
    pub fn new(min: Point3<Float>, max: Point3<Float>) -> AABB {
        assert!(min[0] <= max[0], "Constructed aabb is not aligned on x axis");
        assert!(min[1] <= max[1], "Constructed aabb is not aligned on y axis");
        assert!(min[2] <= max[2], "Constructed aabb is not aligned on z axis");

        Self::new_unchecked(min, max)
    }
    pub fn new_unchecked(min: Point3<Float>, max: Point3<Float>) -> AABB {
        AABB {
            min,
            max
        }
    }
    pub fn from_points(points: &[Point3<Float>]) -> Self {
        let mut min = Point3::new(Float::MAX, Float::MAX, Float::MAX);
        let mut max = Point3::new(Float::MIN, Float::MIN, Float::MIN);

        for point in points {
            for a in 0..3 {
                min[a] = min[a].min(point[a]);
                max[a] = max[a].max(point[a]);
            }
        }

        Self::new(min, max)
    }
    pub fn merged(a: AABB, b: AABB) -> Self {
        Self::from_points(&[a.min, a.max, b.min, b.max])
    }

    pub fn is_flat(&self) -> bool {
        for a in 0..3 {
            if self.min[a] == self.max[a] {
                return true;
            }
        }
        false
    }
    pub fn is_misaligned(&self) -> bool {
        for a in 0..3 {
            if self.min[a] > self.max[a] {
                return true;
            }
        }
        false
    }


    pub fn contains(&self, p: &Point3<Float>) -> bool {
        for a in 0..3 {
            if p[a] > self.max[a] || p[a] < self.min[a] {
                return false;
            }
        }

        true
    }

    pub fn intersects_ray(&self, r: &Ray, mut t_min: Float, mut t_max: Float) -> bool {
        if self.is_misaligned() {
            panic!("Tried to test intersection against misaligned aabb");
        }


        for a in 0..3 {
            if self.min[a] == self.max[a] {
                continue;
            }

            let inv_d = 1.0 / r.direction[a];
            let mut t0 = (self.min[a] - r.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - r.origin[a]) * inv_d;

            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }


            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}
