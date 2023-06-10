mod camera;
mod dielectric;
mod hitable;
mod hitable_list;
mod lambertian;
mod material;
mod metal;
mod ray;
mod sphere;
mod utils;
mod vec3;

use crate::camera::Camera;
use crate::dielectric::Dielectric;
use crate::hitable::HitRecord;
use crate::hitable::Hitable;
use crate::hitable_list::HitableList;
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utils::color::Color;
use crate::utils::infinity;
use crate::utils::random_double;
use crate::utils::write_color;
use crate::vec3::Vec3;

use std::sync::Arc;

fn ray_color(r: &Ray, world: &dyn Hitable, depth: usize) -> Color {
    let mut rec = HitRecord::default();

    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    // If hit something
    if world.hit(r, 0.001, infinity, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();

        if rec
            .mat_ptr
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return ray_color(&scattered, world, depth - 1) * attenuation;
        } else {
            return Color::new(0.0, 0.0, 0.0);
        }
    }

    // If hit nothing return background
    let unit_direction = Vec3::unit_vector(&r.direction());
    let t = (unit_direction.y + 1.0) * 0.5;
    return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 640;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    // World
    let mut world = HitableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Dielectric::new(1.5));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));

    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));

    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center));

    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));

    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    // Camera
    let cam = Camera::new();

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);

        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _s in 0..samples_per_pixel {
                let u = (i as f64 + random_double(None)) / (image_width - 1) as f64;
                let v = (j as f64 + random_double(None)) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, max_depth);
            }

            write_color(pixel_color, samples_per_pixel as f64);
        }
    }
}
