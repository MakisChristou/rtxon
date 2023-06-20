use std::sync::Arc;

use crate::{
    hitable::HitRecord,
    material::{Material, ScatterRay},
    ray::Ray,
    solid_color::SolidColor,
    texture::{self, Texture},
    utils::color::Color,
    vec3::Vec3,
};

pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(c: Color) -> Self {
        DiffuseLight {
            emit: Arc::new(SolidColor::new(c)),
        }
    }
    pub fn new_from_texture(emit: Arc<dyn Texture>) -> Self {
        DiffuseLight { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRay> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Color {
        return self.emit.value(u, v, p);
    }
}
