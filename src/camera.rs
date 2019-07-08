use std::f64::consts;

use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * consts::PI / 180.;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).into_unit();
        let u = vup.cross(&w).into_unit();
        let v = w.cross(&u);
        Camera {
            lower_left_corner: lookfrom - half_width * u - half_height * v - w,
            horizontal: 2. * half_width * u,
            vertical: 2. * half_height * v,
            origin: lookfrom,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
