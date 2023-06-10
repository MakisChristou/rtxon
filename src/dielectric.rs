use crate::{hitable::HitRecord, material::Material, ray::Ray, utils::color::Color, vec3::Vec3};

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Dielectric { ir }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);

        let reflection_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = Vec3::unit_vector(&r_in.direction());

        let cos_theta = f64::min(Vec3::dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = reflection_ratio * sin_theta > 1.0;
        let mut direction = Vec3::new(0.0, 0.0, 0.0);

        if cannot_refract {
            direction = Vec3::reflect(&unit_direction, &rec.normal);
        } else {
            direction = Vec3::refract(&unit_direction, &rec.normal, reflection_ratio);
        }

        *scattered = Ray::new(rec.p, direction);

        return true;
    }
}
