use crate::{ray::Ray, vec3::Vec3};

struct AxisAlignedBoundingBox {
    minimum: Vec3,
    maximum: Vec3,
}

impl AxisAlignedBoundingBox {
    pub fn new(minimum: Vec3, maximum: Vec3) -> Self {
        AxisAlignedBoundingBox { minimum, maximum }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
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
                return false;
            }
        }
        true
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
}
