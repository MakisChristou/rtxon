use std::sync::{mpsc, Arc};

use crate::{
    camera::Camera,
    config::Config,
    hitable::Hitable,
    hitable_list::HitableList,
    material::ScatterRay,
    ray::Ray,
    thread_pool::ThreadPool,
    // thread_pool::Job,
    utils::{color::Color, get_corrected_color, random_double, INFINITY},
    vec3::Vec3,
};
use image::ImageError;
use indicatif::ProgressBar;

#[derive(Debug)]
struct RenderedPixels {
    range: (usize, usize),
    pixels: Vec<Color>,
}

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

    pub fn save(&self, file_path: &str) -> Result<(), ImageError> {
        Self::save_image(
            &self.pixel_colours,
            self.config.image_width,
            self.config.image_height,
            file_path,
        )
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

        return *background;
    }

    pub fn render_current_frame(&mut self, background: &Color) {
        // For updating the progress bar
        let mut rendered = 0;

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
    }

    pub fn render_current_frame_threadpool(
        &mut self,
        background: Color,
        threads: usize,
        chunks: usize,
    ) {
        let thread_pool = ThreadPool::new(threads);
        let (result_sender, result_receiver) = mpsc::channel::<RenderedPixels>();

        let ranges = Renderer::get_ranges(
            0,
            self.config.image_height * self.config.image_width,
            chunks,
        );
        for range in ranges {
            println!("{}, {}", range.0, range.1);

            let result_sender = result_sender.clone();
            let cloned_world = self.world.clone();
            let config = self.config.clone();
            let cam = self.cam.clone();
            let background = background.clone();
            let bg = background.clone();
            thread_pool.execute(move || {
                let pixels = Renderer::render_range(config, cloned_world, cam, bg, range);
                result_sender
                    .send(RenderedPixels { range, pixels })
                    .unwrap();
            });
        }

        // Receive the results
        let mut got = 0;
        let mut results = Vec::new();
        while let Ok(result) = result_receiver.recv() {
            results.push(result);
            got += 1;
            if got == chunks {
                break;
            }
        }

        // Now, 'results' contains all the RenderedPixels
        println!("Received {} results", results.len());

        Renderer::sort_rendered_pixels(&mut results);

        self.pixel_colours.clear();

        for mut result in results {
            println!("{:?}", result.range);
            self.pixel_colours.append(&mut result.pixels);
        }
    }

    fn get_ranges(start: usize, end: usize, chunks: usize) -> Vec<(usize, usize)> {
        assert!(end > start, "end must be greater than start");
        assert!(chunks > 0, "number of chunks must be positive");

        let total_elements = end - start;
        let chunk_size = total_elements / chunks;
        let remainder = total_elements % chunks;

        let mut ranges = Vec::new();
        let mut current_start = start;

        for i in 0..chunks {
            let additional = if i < remainder { 1 } else { 0 };
            let current_end = current_start + chunk_size + additional;

            ranges.push((current_start, current_end));
            current_start = current_end;
        }

        ranges
    }

    fn sort_rendered_pixels(rendered_pixels: &mut Vec<RenderedPixels>) {
        rendered_pixels.sort_by(|a, b| a.range.cmp(&b.range));
    }

    fn render_range(
        config: Config,
        world: impl Hitable,
        cam: Camera,
        background: Color,
        range: (usize, usize),
    ) -> Vec<Color> {
        assert!(range.1 - range.0 <= config.image_width * config.image_height);
        let mut result = Vec::with_capacity(range.1 - range.0);

        for k in range.0..range.1 {
            let i = k % config.image_width;
            let j = k / config.image_width;

            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _s in 0..config.samples_per_pixel {
                let u = (i as f64 + random_double(None)) / (config.image_width - 1) as f64;
                let v = (j as f64 + random_double(None)) / (config.image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color =
                    pixel_color + Self::ray_color(&r, &background, &world, config.max_depth);
            }
            result.push(get_corrected_color(
                pixel_color,
                config.samples_per_pixel as f64,
            ));
        }
        result
    }
}
