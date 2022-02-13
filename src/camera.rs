use nalgebra::{Point3, Unit, Vector3};
use crate::Float;
use crate::ray::Ray;

pub struct Camera {
    origin: Point3<Float>,
    lower_left_corner: Point3<Float>,
    horizontal: Vector3<Float>,
    vertical: Vector3<Float>,
}
impl Camera {
    pub fn new(
        look_from: Point3<Float>,
        look_at: Point3<Float>,
        up: Vector3<Float>,
        vfov_radians: Float,
        aspect_ratio: Float,
    ) -> Self {
        let theta = vfov_radians;
        let h = (theta / 2.0).tan();
        let viewport_height = h * 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (&look_from - look_at).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = &origin
            - horizontal / 2.0
            - vertical / 2.0
            - w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
    pub fn get_ray(&self, s: Float, t: Float) -> Ray {
        let origin = self.origin.clone();
        let dir = &self.lower_left_corner
            + &self.horizontal * s
            + &self.vertical * t
            - &origin;

        Ray::new(origin, Unit::new_normalize(dir))
    }
}
