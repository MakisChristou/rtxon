use std::process;

use crate::{hitable::HitRecord, material::Material, ray::Ray, utils::color::Color, vec3::Vec3};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(&Vec3::unit_vector(&r_in.direction()), &rec.normal);

        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;

        return Vec3::dot(&scattered.direction(), &rec.normal) > 0.0;
    }
}
