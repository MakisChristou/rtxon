use std::sync::Arc;

use crate::{utils::color::Color, vec3::Vec3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}
