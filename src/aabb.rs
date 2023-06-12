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
}
