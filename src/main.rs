mod camera;
mod material;
mod object;
mod ray;
mod sphere;
mod vec3;

use rand::prelude::*;

use crate::camera::Camera;
use crate::material::Material;
use crate::object::Hitable;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

fn color(r: &Ray, world: &impl Hitable, rng: &mut SmallRng, depth: i64) -> Vec3 {
    if let Some(hr) = world.hit(r, 0.001, std::f64::MAX) {
        if depth >= 50 {
            return Vec3(0., 0., 0.);
        }
        if let Some((scattered, attenuation)) = hr.material.scatter(r, &hr, rng) {
            return attenuation * color(&scattered, world, rng, depth + 1);
        } else {
            return Vec3(0., 0., 0.);
        }
    } else {
        let unit_direction = r.direction.into_unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0);
    }
}

fn main() -> std::io::Result<()> {
    let nx = 1000;
    let ny = 500;
    let ns = 100;
    let mut imgbuf = image::ImageBuffer::new(nx, ny);
    let mut rng = rand::rngs::SmallRng::seed_from_u64(0xDEADBEEF);
    let world: Vec<Box<Hitable>> = vec![
        Box::new(Sphere {
            center: Vec3(0.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::Lambertian {
                albedo: Vec3(0.8, 0.3, 0.3),
            } as Material,
        }),
        Box::new(Sphere {
            center: Vec3(0.0, -100.5, -1.0),
            radius: 100.0,
            material: Material::Lambertian {
                albedo: Vec3(0.8, 0.8, 0.0),
            } as Material,
        }),
        Box::new(Sphere {
            center: Vec3(1.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::Metal {
                albedo: Vec3(0.8, 0.6, 0.2),
                fuzz: 1.0,
            } as Material,
        }),
        Box::new(Sphere {
            center: Vec3(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::Metal {
                albedo: Vec3(0.8, 0.8, 0.8),
                fuzz: 0.3,
            } as Material,
        }),
    ];
    let cam = Camera::new();
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let i = x;
        let j = ny - y;
        let mut col = Vec3(0.0, 0.0, 0.0);
        for _ in 0..ns {
            let u = ((i as f64) + rng.gen::<f64>()) / (nx as f64);
            let v = ((j as f64) + rng.gen::<f64>()) / (ny as f64);
            let r = cam.get_ray(u, v);
            col += color(&r, &world, &mut rng, 0);
        }
        col /= ns as f64;
        col = Vec3(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());
        *pixel = image::Rgb([
            (255.99 * col.r()) as u8,
            (255.99 * col.g()) as u8,
            (255.99 * col.b()) as u8,
        ])
    }
    imgbuf.save("image.png")?;
    Ok(())
}
