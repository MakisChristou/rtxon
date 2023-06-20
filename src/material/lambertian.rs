use std::sync::Arc;

use crate::{
    hitable::HitRecord,
    material::{Material, ScatterRay},
    ray::Ray,
    texture::solid_color::SolidColor,
    texture::Texture,
    utils::color::Color,
    vec3::Vec3,
};

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        let solid_color = SolidColor::new(albedo);
        Lambertian {
            albedo: Arc::new(solid_color),
        }
    }

    pub fn new_from_texture(texture: Arc<dyn Texture>) -> Self {
        Lambertian { albedo: texture }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRay> {
        let mut scatter_direction = rec.normal + Vec3::random_in_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let ray = Ray::new_with_time(rec.p, scatter_direction, r_in.time);
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        Some(ScatterRay { ray, attenuation })
    }
}
