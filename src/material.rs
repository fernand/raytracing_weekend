use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 },
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        match self {
            Material::Lambertian { albedo } => {
                let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
                Some((Ray::new(rec.p, target - rec.p), *albedo))
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = r_in.direction.into_unit().reflect(&rec.normal);
                let scattered = Ray::new(rec.p, reflected + *fuzz * Vec3::random_in_unit_sphere());
                if scattered.direction.dot(&rec.normal) > 0. {
                    Some((scattered, *albedo))
                } else {
                    None
                }
            }
        }
    }
}
