use crate::{ray::Ray, vec3::Vec3};


struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f64,
}

trait Hitable {
    fn hit(r: &Ray, t_min: f64, t_max: f64, rec: &HitRecord) -> bool;
}