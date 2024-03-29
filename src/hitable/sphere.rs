use std::sync::Arc;

use crate::{
    aabb::Aabb,
    hitable::{HitRecord, Hitable},
    material::Material,
    ray::Ray,
    utils::PI,
    vec3::Vec3,
};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat_ptr: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            mat_ptr,
        }
    }

    pub fn get_sphere_uv(p: &Vec3) -> (f64, f64) {
        let theta = f64::acos(-p.y);
        let phi = f64::atan2(-p.z, p.x) + PI;
        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());

        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec = HitRecord::default();
        rec.t = root;
        rec.p = r.at(rec.t);

        let outward_normal = (rec.p - self.center) / self.radius;
        (rec.u, rec.v) = Self::get_sphere_uv(&outward_normal);
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = Arc::clone(&self.mat_ptr);

        Some(rec)
    }

    fn bounding_box(&self, time: (f64, f64)) -> Option<crate::aabb::Aabb> {
        Some(Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}
