mod camera;
mod object;
mod ray;
mod sphere;
mod vec3;

use std::fs::File;
use std::io::Write;

use rand::prelude::*;

use crate::camera::Camera;
use crate::object::Hitable;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Float, Vec3, MAX_FLOAT};

fn color(r: &Ray, world: &impl Hitable, rng: &mut impl Rng) -> Vec3 {
    if let Some(hr) = world.hit(r, 0.0, MAX_FLOAT) {
        let target = hr.p + hr.normal + Vec3::random_in_unit_sphere(rng);
        return 0.5 * color(&Ray::new(hr.p, target - hr.p), world, rng);
    } else {
        let unit_direction = r.direction.into_unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0);
    }
}

fn main() -> std::io::Result<()> {
    let nx: i64 = 2000;
    let ny: i64 = 1000;
    let ns: i64 = 100;
    let mut file = File::create("hello.ppm")?;
    let headers = format!("P3\n{} {}\n255\n", nx, ny);
    file.write(headers.as_bytes())?;
    let mut rng = rand::rngs::SmallRng::seed_from_u64(0xDEADBEEF);
    let world: Vec<Box<Hitable>> = vec![
        Box::new(Sphere {
            center: Vec3(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: Vec3(0.0, -100.5, -1.0),
            radius: 100.0,
        }),
    ];
    let cam = Camera::new();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = ((i as Float) + rng.gen::<Float>()) / (nx as Float);
                let v = ((j as Float) + rng.gen::<Float>()) / (ny as Float);
                let r = cam.get_ray(u, v);
                col += color(&r, &world, &mut rng);
            }
            col /= ns as Float;
            file.write(
                format!(
                    "{} {} {}\n",
                    (255.99 * col.r()) as i64,
                    (255.99 * col.g()) as i64,
                    (255.99 * col.b()) as i64
                )
                .as_bytes(),
            )?;
        }
    }
    Ok(())
}
