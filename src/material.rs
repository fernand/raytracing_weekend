use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::rngs::SmallRng;

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

pub enum Material {
    Lambertian { albedo: Vec3 },
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut SmallRng) -> Option<(Ray, Vec3)> {
        match self {
            Material::Lambertian { albedo } => {
                let target = rec.p + rec.normal + Vec3::random_in_unit_sphere(rng);
                Some((Ray::new(rec.p, target - rec.p), *albedo))
            }
        }
    }
}
