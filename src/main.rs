mod ray;
mod utils;
mod vec3;

use ray::Ray;
use vec3::Vec3;

use crate::utils::color::Color;
use crate::utils::write_color;

fn ray_color(r: &Ray) -> Color {
    // Random red sphere
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, &r) {
        return Color::new(1.0, 0.0, 0.0);
    }

    // Render background
    let unit_direction = Vec3::unit_vector(&r.direction());
    let t = 0.5 * unit_direction.y + 1.0;
    return Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    } * (1.0 - t)
        + Color {
            r: 0.5,
            g: 0.7,
            b: 1.0,
        } * t;
}

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - *center;
    let a = Vec3::dot(&r.direction(), &r.direction());
    let b = 2.0 * Vec3::dot(&oc, &r.direction());
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
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

            let pixel_color = ray_color(&r);

            let c = Color {
                r: (i as f64) / (image_width as f64 - 1.0),
                g: (j as f64) / (image_height as f64 - 1.0),
                b: 0.25,
            };

            write_color(pixel_color);
        }
    }
}
