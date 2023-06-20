pub mod checker_texture;
pub mod solid_color;

use crate::{utils::color::Color, vec3::Vec3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}
