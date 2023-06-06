mod vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);

        for i in 0..image_width {
            let r: f32 = (i as f32) / (image_width as f32 - 1.0);
            let g: f32 = (j as f32) / (image_height as f32 - 1.0);
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
