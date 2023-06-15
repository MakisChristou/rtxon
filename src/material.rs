use crate::{hitable::HitRecord, ray::Ray, utils::color::Color, vec3::Vec3};

pub struct ScatterRay {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRay>;
    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
