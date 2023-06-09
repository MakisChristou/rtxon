mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod utils;
mod vec3;

use hitable::HitRecord;
use ray::Ray;
use vec3::Vec3;

use crate::hitable::Hitable;
use crate::hitable_list::HitableList;
use crate::sphere::Sphere;
use crate::utils::color::Color;
use crate::utils::infinity;
use crate::utils::write_color;

fn ray_color(r: &Ray, world: &dyn Hitable) -> Color {
    let mut rec = HitRecord::default();
    // If hit something
    if world.hit(r, 0.0, infinity, &mut rec) {
        return (Color::from(rec.normal) + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }

    // If hit nothing return background
    let unit_direction = Vec3::unit_vector(&r.direction());
    let t = (unit_direction.y + 1.0) * 0.5;
    return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
}

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b: f64 = Vec3::dot(&oc, &r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - f64::sqrt(discriminant)) / a;
    }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    // World
    let mut world = HitableList::new();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);

        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );

            let pixel_color = ray_color(&r, &world);

            write_color(pixel_color);
        }
    }
}
