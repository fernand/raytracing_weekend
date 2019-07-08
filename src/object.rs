use crate::material::HitRecord;
use crate::ray::Ray;

pub trait Hitable: Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl Hitable for Vec<Box<Hitable>> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;
        for object in self.iter() {
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
