use std::rc::Rc;

use crate::{
    aabb::AxisAlignedBoundingBox,
    hitable::{HitRecord, Hitable},
    material::Material,
    vec3::Vec3,
};

pub struct XZRectangle {
    x: (f64, f64),
    z: (f64, f64),
    k: f64,
    material: Rc<dyn Material>,
}

impl XZRectangle {
    pub fn new(x: (f64, f64), z: (f64, f64), k: f64, material: Rc<dyn Material>) -> Self {
        XZRectangle { x, z, k, material }
    }
}

impl Hitable for XZRectangle {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.y) / r.direction.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin.x + t * r.direction.x;
        let z = r.origin.z + t * r.direction.z;
        if x < self.x.0 || x > self.x.1 || z < self.z.0 || z > self.z.1 {
            return None;
        }

        let mut rec = HitRecord::default();

        rec.u = (x - self.x.0) / (self.x.1 - self.x.0);
        rec.v = (z - self.z.0) / (self.z.1 - self.z.0);
        rec.t = t;

        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(r, &outward_normal);

        rec.mat_ptr = Rc::clone(&self.material);
        rec.p = r.at(t);

        Some(rec)
    }

    fn bounding_box(&self, time: (f64, f64)) -> Option<AxisAlignedBoundingBox> {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount.
        Some(AxisAlignedBoundingBox {
            minimum: Vec3::new(self.x.0, self.z.0, self.k - 0.0001),
            maximum: Vec3::new(self.x.1, self.z.1, self.k + 0.0001),
        })
    }
}
