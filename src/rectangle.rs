use std::sync::Arc;

use crate::{
    aabb::AxisAlignedBoundingBox,
    hitable::{HitRecord, Hitable},
    material::Material,
    vec3::Vec3,
};

pub struct Rectangle {
    x: (f64, f64),
    y: (f64, f64),
    k: f64,
    material: Arc<dyn Material>,
}

impl Rectangle {
    pub fn new(x: (f64, f64), y: (f64, f64), k: f64, material: Arc<dyn Material>) -> Self {
        Rectangle { x, y, k, material }
    }
}

impl Hitable for Rectangle {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = self.k - r.origin.z / r.direction.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin.x + t * r.direction.x;
        let y = r.origin.y + t * r.direction.y;
        if x < self.x.0 || x > self.x.1 || y < self.y.0 || y > self.y.1 {
            return None;
        }

        let mut rec = HitRecord::default();

        rec.u = (x - self.x.0) / (self.x.1 - self.x.0);
        rec.v = (y - self.y.0) / (self.y.1 - self.y.0);
        rec.t = t;

        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r, &outward_normal);

        rec.mat_ptr = Arc::clone(&self.material);
        rec.p = r.at(t);

        Some(rec)
    }

    fn bounding_box(&self, time: (f64, f64)) -> Option<AxisAlignedBoundingBox> {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount.
        Some(AxisAlignedBoundingBox {
            minimum: Vec3::new(self.x.0, self.y.0, self.k - 0.0001),
            maximum: Vec3::new(self.x.1, self.y.1, self.k + 0.0001),
        })
    }
}
