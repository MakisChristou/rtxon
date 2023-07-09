use clap::Parser;

/// A Raytracer In One Weekend implementation
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Size of the tuple in decimal digits
    #[arg(short, long, default_value_t = 600)]
    pub width: usize,

    /// Samples per pixel
    #[arg(short, long, default_value_t = 128)]
    pub samples: i32,

    /// Primorial offset
    #[arg(short, long, default_value_t = 100)]
    pub max_depth: usize,

    /// Desired output location
    #[arg(short, long, default_value_t = String::from("output.png"))]
    pub output_path: String,

    /// Threads
    #[arg(short, long, default_value_t = 1)]
    pub threads: usize,

    /// Threadpool Chunks
    #[arg(short, long, default_value_t = 1)]
    pub chucks: usize,
}
