use crate::{
    hitable::HitRecord,
    material::{Material, ScatterRay},
    ray::Ray,
    utils::color::Color,
    vec3::Vec3,
};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
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
        let attenuation = self.albedo;
        Some(ScatterRay { ray, attenuation })
    }
}
