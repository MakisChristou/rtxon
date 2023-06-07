pub mod color;

use crate::utils::color::Color;

pub fn write_color(c: Color) {
    let ir = (255.999 * c.r) as i32;
    let ig = (255.999 * c.g) as i32;
    let ib = (255.999 * c.b) as i32;
    println!("{} {} {}", ir, ig, ib);
}
