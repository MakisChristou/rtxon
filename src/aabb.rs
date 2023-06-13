use crate::{
    hitable::{HitRecord, Hitable},
    ray::Ray,
    vec3::Vec3,
};

#[derive(Copy, Clone)]
pub struct AxisAlignedBoundingBox {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl Hitable for AxisAlignedBoundingBox {
    // We don't care about returning actual values in the hit record just that we hit SOME record
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        for a in 0..3 {
            let t0 = f64::min(
                self.minimum.at(a).unwrap() - r.origin.at(a).unwrap() / r.direction.at(a).unwrap(),
                self.maximum.at(a).unwrap() - r.origin.at(a).unwrap() / r.direction.at(a).unwrap(),
            );
            let t1 = f64::max(
                self.minimum.at(a).unwrap() - r.origin.at(a).unwrap() / r.direction.at(a).unwrap(),
                self.maximum.at(a).unwrap() - r.origin.at(a).unwrap() / r.direction.at(a).unwrap(),
            );

            let t_min = f64::max(t0, t_min);
            let t_max = f64::min(t1, t_max);

            if t_max <= t_min {
                return None;
            }
        }
        Some(HitRecord::default())
    }

    fn bounding_box(&self, time: (f64, f64)) -> Option<AxisAlignedBoundingBox> {
        todo!()
    }
}

impl AxisAlignedBoundingBox {
    pub fn new(minimum: Vec3, maximum: Vec3) -> Self {
        AxisAlignedBoundingBox { minimum, maximum }
    }

    pub fn andrew_kensler_hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction.at(a).unwrap();
            let mut t0 = self.minimum.at(a).unwrap() - r.origin.at(a).unwrap() * inv_d;
            let mut t1 = self.maximum.at(a).unwrap() - r.origin.at(a).unwrap() * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(
        box0: AxisAlignedBoundingBox,
        box1: AxisAlignedBoundingBox,
    ) -> AxisAlignedBoundingBox {
        let small = Vec3::new(
            f64::min(box0.minimum.x, box1.minimum.x),
            f64::min(box0.minimum.y, box1.minimum.y),
            f64::min(box0.minimum.z, box1.minimum.z),
        );

        let big = Vec3::new(
            f64::min(box0.maximum.x, box1.maximum.x),
            f64::min(box0.maximum.y, box1.maximum.y),
            f64::min(box0.maximum.z, box1.maximum.z),
        );

        AxisAlignedBoundingBox {
            minimum: small,
            maximum: big,
        }
    }
}
