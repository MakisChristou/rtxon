mod camera;
mod hitable;
mod hitable_list;
mod lambertian;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use std::sync::Arc;

use camera::Camera;
use hitable::HitRecord;
use ray::Ray;
use vec3::Vec3;

use crate::hitable::Hitable;
use crate::hitable_list::HitableList;
use crate::lambertian::Lambertian;
use crate::sphere::Sphere;
use crate::utils::color::Color;
use crate::utils::infinity;
use crate::utils::random_double;
use crate::utils::write_color;

fn ray_color(r: &Ray, world: &dyn Hitable, depth: usize) -> Color {
    let mut rec = HitRecord::default();

    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    // If hit something
    if world.hit(r, 0.001, infinity, &mut rec) {
        // Bounce
        let target = rec.p + rec.normal + Vec3::random_in_hemisphere(&rec.normal);
        return ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1) * 0.5;
    }

    // If hit nothing return background
    let unit_direction = Vec3::unit_vector(&r.direction());
    let t = (unit_direction.y + 1.0) * 0.5;
    return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 1280;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    // World
    let mut world = HitableList::new();
    world.add(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Arc::new(Lambertian::new(Color::new(255.0, 0.0, 0.0))),
    ));
    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Arc::new(Lambertian::new(Color::new(125.0, 125.0, 125.0))),
    ));

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
