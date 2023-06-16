use std::rc::Rc;

use crate::{utils::color::Color, vec3::Vec3};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}
