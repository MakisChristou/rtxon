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
use crate::utils::get_corrected_color;
use crate::utils::infinity;
use crate::utils::random_double;
use crate::vec3::Vec3;

use image::ImageError;
use indicatif::ProgressBar;
use indicatif::ProgressState;
use indicatif::ProgressStyle;
use std::fmt::Write;
use std::sync::Arc;
use utils::pi;

fn save_image(
    pixel_colours: &Vec<Color>,
    width: usize,
    height: usize,
    file_path: &str,
) -> Result<(), ImageError> {
    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let c = pixel_colours[(height - 1 - y as usize) * width + x as usize]; // Flip the y-axis here
        *pixel = image::Rgb([c.r as u8, c.g as u8, c.b as u8]);
    }

    imgbuf.save(file_path)
}

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

fn random_scene() -> (HitableList, Camera) {
    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let mut world = HitableList::new();
    world.add(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;

            let choose_mat = random_double(None);
            let center = Vec3::new(
                a + 0.9 * random_double(None),
                0.2,
                b + 0.9 * random_double(None),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(None) * Color::random(None);
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(Some((0.5, 1.0)));
                    let fuzz = random_double(Some((0.0, 0.5)));
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    // Main scene
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0; //(look_from - look_at).length();
    let appreture = 0.1;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        appreture,
        dist_to_focus,
    );

    return (world, cam);
}

fn scene1() -> (HitableList, Camera) {
    let mut world = HitableList::new();
    let r = f64::cos(pi / 4.0);

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));

    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));

    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center));
    world.add(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    ));

    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, material_left));

    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    let aspect_ratio = 16.0 / 9.0;
    let look_from = Vec3::new(3.0, 3.0, 2.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let appreture = 2.0;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        appreture,
        dist_to_focus,
    );

    return (world, cam);
}

fn scene2() -> (HitableList, Camera) {
    let mut world = HitableList::new();
    let r = f64::cos(pi / 4.0);

    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(1.0, 0.2, 0.3), 0.1));

    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));

    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center));
    world.add(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    ));
    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    let aspect_ratio = 16.0 / 9.0;
    let look_from = Vec3::new(0.0, 0.5, 1.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let appreture = 0.05;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        60.0,
        aspect_ratio,
        appreture,
        dist_to_focus,
    );

    return (world, cam);
}

fn scene3() -> (HitableList, Camera) {
    let mut world = HitableList::new();
    let r = f64::cos(pi / 4.0);

    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let material_center = Arc::new(Lambertian::new(Color::new(1.0, 1.0, 1.0)));
    let material_left = Arc::new(Lambertian::new(Color::new(0.2, 0.3, 1.0)));
    let material_right = Arc::new(Metal::new(Color::new(1.0, 0.2, 0.3), 0.1));

    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    ));

    world.add(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center.clone(),
    ));
    world.add(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    ));
    world.add(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_ground.clone(),
    ));

    let aspect_ratio = 16.0 / 9.0;
    let look_from = Vec3::new(0.0, 0.5, 1.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let appreture = 0.05;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        60.0,
        aspect_ratio,
        appreture,
        dist_to_focus,
    );

    return (world, cam);
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 1920;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 200;
    let max_depth = 100;

    // World && Camera
    let (world, cam) = scene3();

    // Image Buffer
    let mut pixel_colours: Vec<Color> = vec![Color::new(0.0, 0.0, 0.0); image_height * image_width];

    // Progress Bar
    let mut rendered = 0;
    let total_size = image_height * image_width;

    let pb = ProgressBar::new(total_size as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] ({eta})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _s in 0..samples_per_pixel {
                let u = (i as f64 + random_double(None)) / (image_width - 1) as f64;
                let v = (j as f64 + random_double(None)) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, max_depth);
            }

            rendered += 1;
            pb.set_position(rendered);

            pixel_colours[j * image_width + i] =
                get_corrected_color(pixel_color, samples_per_pixel as f64);
        }
    }

    save_image(&pixel_colours, image_width, image_height, "output.png");
}
