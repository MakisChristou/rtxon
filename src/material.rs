use crate::{hitable::HitRecord, ray::Ray, utils::color::Color};

pub struct ScatterRay {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRay>;
}
