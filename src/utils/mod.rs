pub mod color;

use crate::{
    hitable::{hitable_list::HitableList, triangle::Triangle},
    material::Material,
    utils::color::Color,
    vec3::Vec3,
};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::{cell::RefCell, path::Path, sync::Arc};
use tobj::LoadOptions;

// Static variables
pub static INFINITY: f64 = std::f64::INFINITY;
pub static PI: f64 = std::f64::consts::PI;

pub fn write_color(c: Color, samples_per_pixel: f64) {
    let mut r = c.r;
    let mut g = c.g;
    let mut b = c.b;

    let scale = 1.0 / samples_per_pixel;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    let ir = (256.0 * clamp(r, 0.0, 0.999)) as i32;
    let ig = (256.0 * clamp(g, 0.0, 0.999)) as i32;
    let ib = (256.0 * clamp(b, 0.0, 0.999)) as i32;
    println!("{} {} {}", ir, ig, ib);
}

pub fn get_corrected_color(c: Color, samples_per_pixel: f64) -> Color {
    let mut r = c.r;
    let mut g = c.g;
    let mut b = c.b;

    let scale = 1.0 / samples_per_pixel;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    let ir = 256.0 * clamp(r, 0.0, 0.999);
    let ig = 256.0 * clamp(g, 0.0, 0.999);
    let ib = 256.0 * clamp(b, 0.0, 0.999);

    Color::new(ir, ig, ib)
}

pub fn random_int(start: usize, end: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(start..end)
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn obj_import_as_triangles(path: &str, material: Arc<dyn Material>) -> HitableList {
    let mut world = HitableList::new();

    let obj_file = Path::new(path);

    let (models, _) =
        tobj::load_obj(&obj_file, &LoadOptions::default()).expect("Failed to load file");

    for (_, model) in models.iter().enumerate() {
        let mesh = &model.mesh;

        // Mesh's indices are organized as triplets, so we'll
        // take them three at a time
        for triangle in mesh.indices.chunks(3) {
            if let [v1, v2, v3] = *triangle {
                let v1 = mesh.positions[(v1 as usize) * 3..(v1 as usize) * 3 + 3].to_vec();
                let v2 = mesh.positions[(v2 as usize) * 3..(v2 as usize) * 3 + 3].to_vec();
                let v3 = mesh.positions[(v3 as usize) * 3..(v3 as usize) * 3 + 3].to_vec();

                let v1 = Vec3::new(v1[0] as f64, v1[1] as f64, v1[2] as f64);
                let v2 = Vec3::new(v2[0] as f64, v2[1] as f64, v2[2] as f64);
                let v3 = Vec3::new(v3[0] as f64, v3[1] as f64, v3[2] as f64);

                world.add(Triangle::new(v1, v2, v3, material.clone()));
            }
        }
    }

    world
}

thread_local! {
    static RNG: RefCell<rand::rngs::SmallRng> = RefCell::new(SmallRng::from_entropy());
}

// Random Number Utilities
pub fn random_double(range: Option<(f64, f64)>) -> f64 {
    RNG.with(|rng| match range {
        Some(range) => rng.borrow_mut().gen_range(range.0..range.1),
        None => rng.borrow_mut().gen_range(0.0..1.0),
    })
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::{random_double, random_int};

    #[test]
    fn should_return_correct_default_random_numnber() {
        let r = random_double(None);
        assert!(r < 1.0 && r > 0.0);
    }

    #[test]
    fn should_choose_only_3_axis() {
        let r = random_int(0, 3);
        assert!(r <= 2 && r >= 0);
    }
}
