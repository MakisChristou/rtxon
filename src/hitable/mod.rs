pub mod hitable_list;
pub mod moving_sphere;
pub mod sphere;
pub mod triangle;
pub mod xy_rectangle;
pub mod xz_rectangle;
pub mod yz_rectangle;

use std::sync::Arc;

use crate::{
    aabb::Aabb,
    material::{lambertian::Lambertian, Material},
    ray::Ray,
    utils::color::Color,
    vec3::Vec3,
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat_ptr: Arc<dyn Material>,
}

impl HitRecord {
    pub fn default() -> Self {
        HitRecord {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
            mat_ptr: Arc::new(Lambertian::new(Color::new(255.0, 0.0, 0.0))),
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hitable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time: (f64, f64)) -> Option<Aabb>;
}
