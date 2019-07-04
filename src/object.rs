use crate::ray::*;
use crate::vec3::*;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub t: Float,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}

impl Hitable for Vec<Box<Hitable>> {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;
        for object in self.iter() {
            if let Some(candidate_hit) = object.hit(r, t_min, t_max) {
                match hit {
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
