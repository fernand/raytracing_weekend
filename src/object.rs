use crate::material::{HitRecord, Material};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub enum Hitable {
    Sphere {
        center: Vec3,
        radius: f64,
        material: Material,
    },
}

impl Hitable {
    fn hit<'a>(&'a self, r: &'a Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Hitable::Sphere {
                center,
                radius,
                material,
            } => {
                let oc = r.origin - *center;
                let a = r.direction.dot(&r.direction);
                let b = oc.dot(&r.direction);
                let c = oc.dot(&oc) - radius * radius;
                let discriminant = b * b - a * c;
                if discriminant > 0.0 {
                    let mut temp = (-b - discriminant.sqrt()) / a;
                    if temp < t_max && temp > t_min {
                        let p = r.point_at_parameter(temp);
                        return Some(HitRecord {
                            t: temp,
                            p: p,
                            normal: (p - *center) / *radius,
                            material: &material,
                        });
                    }
                    temp = (-b + discriminant.sqrt()) / a;
                    if temp < t_max && temp > t_min {
                        let p = r.point_at_parameter(temp);
                        return Some(HitRecord {
                            t: temp,
                            p: p,
                            normal: (p - *center) / *radius,
                            material: &material,
                        });
                    }
                }
                return None;
            }
        }
    }
}

pub struct World {
    objects: Vec<Hitable>,
}

impl World {
    pub fn new(objects: Vec<Hitable>) -> World {
        World { objects: objects }
    }

    pub fn hit<'a>(&'a self, r: &'a Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;
        for object in self.objects.iter() {
            if let Some(candidate_hit) = object.hit(r, t_min, t_max) {
                match &hit {
                    None => hit = Some(candidate_hit),
                    Some(prev) => {
                        if candidate_hit.t < prev.t {
                            hit = Some(candidate_hit);
                        }
                    }
                }
            }
        }
        hit
    }
}
