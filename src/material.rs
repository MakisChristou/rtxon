use crate::{ray::Ray, hitable::HitRecord, utils::color::Color};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}