use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use crate::vec3::Vec3;

    #[test]
    fn should_return_correct_at() {
        let origin = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let direction = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let ray = Ray { origin, direction };
        let point_at = ray.at(10.0);
        assert_eq!(
            point_at,
            Vec3 {
                x: 10.0,
                y: 10.0,
                z: 10.0
            }
        );
    }
}
