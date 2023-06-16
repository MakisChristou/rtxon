use std::rc::Rc;

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
    mat_ptr: Rc<dyn Material>,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, mat_ptr: Rc<dyn Material>) -> Self {
        Triangle { a, b, c, mat_ptr }
    }
}

impl Hitable for Triangle {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;
        let normal = Vec3::unit_vector(&Vec3::cross(&edge1, &edge2));

        // Compute ray and triangle intersection using Möller–Trumbore algorithm
        let p_vec = Vec3::cross(&r.direction, &edge2);
        let det = Vec3::dot(&edge1, &p_vec);

        // If the determinant is near zero, the ray lies in the plane of the triangle
        if (det > -0.0001 && det < 0.0001) {
            return None;
        }

        let inv_det = -1.0 / det;
        let t_vec = r.origin - self.a;
        let u = Vec3::dot(&t_vec, &p_vec) * inv_det;

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q_vec = Vec3::cross(&t_vec, &edge1);
        let v = Vec3::dot(&r.direction, &q_vec) * inv_det;

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = Vec3::dot(&edge2, &q_vec) * inv_det;

        let mut rec = HitRecord::default();

        // if intersection exists, return HitRecord Option
        if t > t_min && t < t_max {
            rec.p = r.at(t);
            rec.normal = normal;
            rec.t = t;
            rec.u = u;
            rec.v = v;
            rec.set_face_normal(r, &normal);
            rec.mat_ptr = Rc::clone(&self.mat_ptr);
            return Some(rec);
        }
        None
    }

    fn bounding_box(&self, time: (f64, f64)) -> Option<AxisAlignedBoundingBox> {
        todo!()
    }
}
