use crate::{
    hitable::HitRecord,
    material::{Material, ScatterRay},
    ray::Ray,
    utils::color::Color,
    vec3::Vec3,
};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        if fuzz < 1.0 {
            Metal { albedo, fuzz }
        } else {
            Metal { albedo, fuzz: 1.0 }
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRay> {
        let reflected = Vec3::reflect(&Vec3::unit_vector(&r_in.direction()), &rec.normal);

        let ray = Ray::new_with_time(
            rec.p,
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
            r_in.time,
        );
        let attenuation = self.albedo;

        if Vec3::dot(&ray.direction(), &rec.normal) > 0.0 {
            Some(ScatterRay { ray, attenuation })
        } else {
            None
        }
    }
}
