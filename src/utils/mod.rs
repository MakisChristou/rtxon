pub mod color;

use crate::utils::color::Color;
use rand::Rng;

// Static variables
pub static infinity: f64 = std::f64::INFINITY;
pub static pi: f64 = std::f64::consts::PI;

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

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * pi / 180.0
}

// Random Number Utilities
pub fn random_double(range: Option<(f64, f64)>) -> f64 {
    match range {
        Some(range) => rand::thread_rng().gen_range(range.0..range.1),
        None => rand::thread_rng().gen_range(0.0..1.0)
    }
    
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

#[cfg(test)]
mod tests {
    use super::random_double;

    #[test]
    fn should_return_correct_default_random_numnber() {
        let r = random_double(None);
        assert!(r < 1.0 && r > 0.0);
    }
}
