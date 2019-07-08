mod camera;
mod material;
mod object;
mod ray;
mod sphere;
mod vec3;

use std::fs::File;
use std::io::BufWriter;

use png::HasParameters;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

use crate::camera::Camera;
use crate::material::Material;
use crate::object::Hitable;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

fn color(r: &Ray, world: &impl Hitable, rng: &mut impl Rng, depth: i64) -> Vec3 {
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
    const NX: usize = 2000;
    const NY: usize = 1000;
    const NS: usize = 100;
    let file = File::create("image.png")?;
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, NX as u32, NY as u32);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
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
    let cam = Camera::new(
        Vec3(-2., 2., 1.),
        Vec3(0., 0., -1.),
        Vec3(0., 1., 0.),
        40.,
        NX as f64 / NY as f64,
    );
    let colors: Vec<Vec<Vec3>> = (0..NY)
        .into_par_iter()
        .rev()
        .map(|j| {
            (0..NX)
                .map(|i| {
                    let mut rng = rand::rngs::SmallRng::seed_from_u64(rand::random::<u64>());
                    let mut col = Vec3(0.0, 0.0, 0.0);
                    for _ in 0..NS {
                        let u = ((i as f64) + rng.gen::<f64>()) / (NX as f64);
                        let v = ((j as f64) + rng.gen::<f64>()) / (NY as f64);
                        let r = cam.get_ray(u, v);
                        col += color(&r, &world, &mut rng, 0);
                    }
                    col /= NS as f64;
                    col
                })
                .collect()
        })
        .collect();
    let mut pixels: Vec<u8> = Vec::new();
    for column in colors.iter() {
        for &col in column.iter() {
            pixels.push((255.99 * col.r().sqrt()) as u8);
            pixels.push((255.99 * col.g().sqrt()) as u8);
            pixels.push((255.99 * col.b().sqrt()) as u8);
        }
    }
    writer.write_image_data(&pixels)?;
    Ok(())
}
