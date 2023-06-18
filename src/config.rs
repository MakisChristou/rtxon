#[derive(Clone)]
pub struct Config {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub image_height: usize,
    pub samples_per_pixel: i32,
    pub max_depth: usize,
}

impl Config {
    pub fn new(
        aspect_ratio: f64,
        image_width: usize,
        samples_per_pixel: i32,
        max_depth: usize,
    ) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as usize;
        Config {
            aspect_ratio,
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
        }
    }
}
