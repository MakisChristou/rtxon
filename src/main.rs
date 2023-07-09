mod aabb;
mod args;
mod bhv;
mod camera;
mod config;
mod hitable;
mod material;
mod ray;
mod renderer;
mod scene;
mod texture;
mod thread_pool;
mod utils;
mod vec3;

use crate::vec3::Vec3;
use clap::Parser;
use config::Config;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use renderer::Renderer;
use std::fmt::Write;

fn main() {
    let args = <args::Args>::parse();

    let scenes = vec![
        scene::random_scene,
        scene::random_moving_scene,
        scene::checker_scene,
        scene::checker_emmisive_material_scene,
        scene::scene1,
        scene::scene2,
        scene::scene3,
        scene::scene4,
        scene::rectangular_light_scene,
        scene::cornell_box_scene,
        scene::teapot_scene,
    ];

    if args.scene >= scenes.len() {
        panic!(
            "Scene {} does not exist there are {} scenes",
            args.scene,
            scenes.len()
        );
    }

    // Scene
    let (world, cam, background, aspect_ratio) = scenes[args.scene]();

    // Image Settings
    let image_width: usize = args.width;
    let samples_per_pixel = args.samples;
    let max_depth = args.max_depth;

    let config = Config::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    // Progress Bar
    let pb = ProgressBar::new(config.image_height as u64 * config.image_width as u64);

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

    let mut renderer = Renderer::new(config, world, cam, Some(pb));

    renderer.render_current_frame_threadpool(background, args.threads, args.chunks);

    match renderer.save(&args.output_path) {
        Ok(()) => {
            println!("Frame saved succesfully")
        }
        Err(e) => panic!("Cannot save frame {}", e),
    }
}
