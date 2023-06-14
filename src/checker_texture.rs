use std::sync::Arc;

use crate::{
    texture::{SolidColor, Texture},
    utils::color::Color,
    vec3::Vec3,
};

pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(c1: Color, c2: Color) -> Self {
        CheckerTexture {
            odd: Arc::new(SolidColor::new(c1)),
            even: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        let sines = f64::sin(10.0 * p.x) * f64::sin(10.0 * p.y) * f64::sin(10.0 * p.z);

        if sines < 0.0 {
            return self.odd.value(u, v, p);
        } else {
            return self.even.value(u, v, p);
        }
    }
}
