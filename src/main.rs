mod camera;
mod material;
mod object;
mod rand;
mod ray;
mod vec3;

use std::fs::File;
use std::io::BufWriter;

use png::HasParameters;

use crate::camera::Camera;
use crate::material::Material;
use crate::object::{Hitable, World};
use crate::ray::Ray;
use crate::vec3::Vec3;

fn color(r: &Ray, world: &World, depth: i64) -> Vec3 {
    if let Some(hr) = world.hit(r, 0.001, std::f64::MAX) {
        if depth >= 50 {
            return Vec3(0., 0., 0.);
        }
        if let Some((scattered, attenuation)) = hr.material.scatter(r, &hr) {
            return attenuation * color(&scattered, world, depth + 1);
        } else {
            return Vec3(0., 0., 0.);
        }
    } else {
        let unit_direction = r.direction.into_unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0);
    }
}

fn write_png(nx: usize, ny: usize, colors: &Vec<Vec<Vec3>>) -> std::io::Result<()> {
    let file = File::create("image.png")?;
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, nx as u32, ny as u32);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
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

fn main() {
    const NX: usize = 2000;
    const NY: usize = 1000;
    const NS: usize = 100;
    let world = World::new(vec![
        Hitable::Sphere {
            center: Vec3(0.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::Lambertian {
                albedo: Vec3(0.8, 0.3, 0.3),
            } as Material,
        },
        Hitable::Sphere {
            center: Vec3(0.0, -100.5, -1.0),
            radius: 100.0,
            material: Material::Lambertian {
                albedo: Vec3(0.8, 0.8, 0.0),
            } as Material,
        },
        Hitable::Sphere {
            center: Vec3(1.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::Metal {
                albedo: Vec3(0.8, 0.6, 0.2),
                fuzz: 1.0,
            } as Material,
        },
        Hitable::Sphere {
            center: Vec3(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::Metal {
                albedo: Vec3(0.8, 0.8, 0.8),
                fuzz: 0.3,
            } as Material,
        },
    ]);
    let cam = Camera::new(
        Vec3(-2., 2., 1.),
        Vec3(0., 0., -1.),
        Vec3(0., 1., 0.),
        40.,
        NX as f64 / NY as f64,
    );
    let mut colors: Vec<Vec<Vec3>> = Vec::new();
    for j in (0..NY).rev() {
        let mut subv: Vec<Vec3> = Vec::new();
        for i in 0..NX {
            let mut col = Vec3(0.0, 0.0, 0.0);
            for _ in 0..NS {
                let u = ((i as f64) + rand::drand()) / (NX as f64);
                let v = ((j as f64) + rand::drand()) / (NY as f64);
                let r = cam.get_ray(u, v);
                col += color(&r, &world, 0);
            }
            col /= NS as f64;
            subv.push(col);
        }
        colors.push(subv);
    }
    write_png(NX, NY, &colors).unwrap();
}
