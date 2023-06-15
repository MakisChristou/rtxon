use crate::{
    camera::Camera,
    config::Config,
    hitable::Hitable,
    hitable_list::HitableList,
    material::ScatterRay,
    ray::Ray,
    utils::{color::Color, get_corrected_color, random_double, INFINITY},
    vec3::Vec3,
};
use image::ImageError;
use indicatif::ProgressBar;

pub struct Renderer {
    config: Config,
    world: HitableList,
    cam: Camera,
    pixel_colours: Vec<Color>,
    pb: Option<ProgressBar>,
}

impl Renderer {
    pub fn new(config: Config, world: HitableList, cam: Camera, pb: Option<ProgressBar>) -> Self {
        let mut pixel_colours: Vec<Color> =
            vec![Color::new(0.0, 0.0, 0.0); config.image_height * config.image_width];
        Renderer {
            config,
            world,
            cam,
            pixel_colours,
            pb,
        }
    }

    fn save_image(
        pixel_colours: &[Color],
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

    fn ray_color(r: &Ray, background: &Color, world: &dyn Hitable, depth: usize) -> Color {
        if depth == 0 {
            return *background;
        }

        // If hit something
        if let Some(rec) = world.hit(r, 0.001, INFINITY) {
            let mut emmited = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p);

            if let Some(ScatterRay { ray, attenuation }) = rec.mat_ptr.scatter(r, &rec) {
                return Self::ray_color(&ray, background, world, depth - 1) * attenuation + emmited;
            } else {
                return emmited;
            }
        }

        return Color::new(0.0, 0.0, 0.0);

        // If hit nothing return background
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = (unit_direction.y + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }

    pub fn render_current_frame(&mut self) -> Result<(), ImageError> {
        // For updating the progress bar
        let mut rendered = 0;
        let background = Color::new(0.0, 0.0, 0.0);

        for j in (0..self.config.image_height).rev() {
            for i in 0..self.config.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _s in 0..self.config.samples_per_pixel {
                    let u = (i as f64 + random_double(None)) / (self.config.image_width - 1) as f64;
                    let v =
                        (j as f64 + random_double(None)) / (self.config.image_height - 1) as f64;
                    let r = self.cam.get_ray(u, v);
                    pixel_color = pixel_color
                        + Self::ray_color(&r, &background, &self.world, self.config.max_depth);
                }

                rendered += 1;

                match &mut self.pb {
                    Some(pb) => {
                        pb.set_position(rendered);
                    }
                    None => {}
                }

                self.pixel_colours[j * self.config.image_width + i] =
                    get_corrected_color(pixel_color, self.config.samples_per_pixel as f64);
            }
        }

        Self::save_image(
            &self.pixel_colours,
            self.config.image_width,
            self.config.image_height,
            "output.png",
        )?;

        Ok(())
    }
}
