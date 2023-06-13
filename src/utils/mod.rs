pub mod color;

use crate::utils::color::Color;
use rand::Rng;

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

// Random Number Utilities
pub fn random_double(range: Option<(f64, f64)>) -> f64 {
    match range {
        Some(range) => rand::thread_rng().gen_range(range.0..range.1),
        None => rand::thread_rng().gen_range(0.0..1.0),
    }
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
