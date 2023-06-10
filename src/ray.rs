use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn default() -> Self {
        Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0))
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
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

    #[test]
    fn should_give_correct_origin() {
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

        assert_eq!(ray.origin(), origin);
    }

    #[test]
    fn should_give_correct_direction() {
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

        assert_eq!(ray.direction(), direction);
    }
}
