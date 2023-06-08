pub mod color;

use crate::utils::color::Color;

// Static variables
static infinity: f64 = std::f64::INFINITY;
static pi: f64 = std::f64::consts::PI;

pub fn write_color(c: Color) {
    let ir = (255.999 * c.r) as i32;
    let ig = (255.999 * c.g) as i32;
    let ib = (255.999 * c.b) as i32;
    println!("{} {} {}", ir, ig, ib);
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * pi / 180.0
}

