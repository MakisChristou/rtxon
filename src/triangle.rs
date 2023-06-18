use std::sync::Arc;

use crate::{
    aabb::AxisAlignedBoundingBox,
    hitable::{HitRecord, Hitable},
    material::Material,
    vec3::Vec3,
};

pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    mat_ptr: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, mat_ptr: Arc<dyn Material>) -> Self {
        Triangle { a, b, c, mat_ptr }
    }
}

impl Hitable for Triangle {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;
        let h = r.direction.cross(&edge2);
        let a = Vec3::dot(&edge1, &h);

        if a > -0.0001 && a < 0.0001 {
            return None; // This ray is parallel to this triangle.
        }

        let f = 1.0 / a;
        let s = r.origin - self.a;
        let u = f * Vec3::dot(&s, &h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * Vec3::dot(&r.direction, &q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = f * Vec3::dot(&edge2, &q);

        if t > 0.0001 && t < t_max && t > t_min {
            // ray intersection
            let p = r.at(t);
            let outward_normal = Vec3::unit_vector(&(self.b - self.a).cross(&(self.c - self.a)));

            let (front_face, normal) = if Vec3::dot(&r.direction, &outward_normal) < 0.0 {
                (true, outward_normal)
            } else {
                (false, -outward_normal)
            };

            Some(HitRecord {
                p,
                normal,
                t,
                u,
                v,
                front_face,
                mat_ptr: Arc::clone(&self.mat_ptr),
            })
        } else {
            None
        }
    }

    fn bounding_box(&self, time: (f64, f64)) -> Option<AxisAlignedBoundingBox> {
        todo!()
    }
}
