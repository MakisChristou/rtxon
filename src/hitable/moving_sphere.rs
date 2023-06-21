use std::sync::Arc;

use crate::{
    aabb::Aabb,
    hitable::{HitRecord, Hitable},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};

pub struct MovingSphere {
    pub center: (Vec3, Vec3),
    pub time: (f64, f64),
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center: (Vec3, Vec3),
        time: (f64, f64),
        radius: f64,
        mat_ptr: Arc<dyn Material>,
    ) -> Self {
        MovingSphere {
            center,
            time,
            radius,
            mat_ptr,
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        self.center.0
            + (self.center.1 - self.center.0) * ((time - self.time.0) / (self.time.1 - self.time.0))
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(time) = r.time {
            let oc = r.origin() - self.center(time);
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

            let outward_normal = (rec.p - self.center(time)) / self.radius;
            rec.set_face_normal(r, &outward_normal);
            rec.mat_ptr = Arc::clone(&self.mat_ptr);

            Some(rec)
        } else {
            panic!("Ray hitting MovingSphere should always have time value");
        }
    }

    fn bounding_box(&self, time: (f64, f64)) -> Option<crate::aabb::Aabb> {
        let box0 = Aabb::new(
            self.center(time.0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time.0) + Vec3::new(self.radius, self.radius, self.radius),
        );

        let box1 = Aabb::new(
            self.center(time.1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time.1) + Vec3::new(self.radius, self.radius, self.radius),
        );

        Some(Aabb::surrounding_box(box0, box1))
    }
}
