use crate::material::{HitRecord, Material};
use crate::object::Hitable;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(&r.direction);
        let b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: p,
                    normal: (p - self.center) / self.radius,
                    material: &self.material,
                });
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: p,
                    normal: (p - self.center) / self.radius,
                    material: &self.material,
                });
            }
        }
        return None;
    }
}
