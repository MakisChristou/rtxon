mod utils;
mod vec3;

use crate::utils::color::Color;
use crate::utils::write_color;

fn main() {
    let image_width = 256;
    let image_height = 256;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);

        for i in 0..image_width {
            let c = Color {
                r: (i as f64) / (image_width as f64 - 1.0),
                g: (j as f64) / (image_height as f64 - 1.0),
                b: 0.25,
            };

            write_color(c);
        }
    }
}
