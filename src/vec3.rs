use std::ops;

use crate::utils::random_double;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn from_one(a: f64) -> Self {
        Vec3 { x: a, y: a, z: a }
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn at(&self, index: usize) -> Option<f64> {
        match index {
            0 => Some(self.x),
            1 => Some(self.y),
            2 => Some(self.z),
            _ => None,
        }
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    pub fn cross(&self, u: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * u.z - self.z * u.y,
            y: self.z * u.x - self.x * u.z,
            z: self.x * u.y - self.y * u.x,
        }
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        let l = v.length();
        Vec3 {
            x: v.x / l,
            y: v.y / l,
            z: v.z / l,
        }
    }

    pub fn random(range: Option<(f64, f64)>) -> Vec3 {
        match range {
            Some(range) => Vec3::new(
                random_double(Some(range)),
                random_double(Some(range)),
                random_double(Some(range)),
            ),
            None => Vec3::new(
                random_double(None),
                random_double(None),
                random_double(None),
            ),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(Some((-1.0, 1.0)));
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_in_unit_vector() -> Vec3 {
        Vec3::unit_vector(&Vec3::random_in_unit_sphere())
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if Vec3::dot(&in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimentions
        let s = 1e-8;
        (f64::abs(self.x) < s) && (f64::abs(self.y) < s) && (f64::abs(self.z) < s)
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        (*v) - (*n) * (2.0 * Vec3::dot(v, n))
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(Vec3::dot(&-(*uv), n), 1.0);
        let r_out_perp = ((*uv) + (*n) * cos_theta) * etai_over_etat;
        let r_out_parallel = (*n) * (-f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())));
        r_out_perp + r_out_parallel
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                random_double(Some((-1.0, 1.0))),
                random_double(Some((-1.0, 1.0))),
                0.0,
            );
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x * _rhs.x, self.y * _rhs.y, self.z * _rhs.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3::new(self.x * _rhs, self.y * _rhs, self.z * _rhs)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f64) -> Vec3 {
        Vec3::new(self.x / _rhs, self.y / _rhs, self.z / _rhs)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    #[test]
    fn should_instantiate_vector() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(
            v,
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }

    #[test]
    fn should_instantiate_from_one() {
        let v = Vec3::from_one(1.0);
        assert_eq!(
            v,
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0
            }
        );
    }

    #[test]
    fn should_have_correct_length_squared() {
        let v = Vec3::new(1.0, 1.0, 5.0);
        assert_eq!(v.length_squared(), 27.0);
    }

    #[test]
    fn should_have_correct_length() {
        let v = Vec3::new(3.0, 2.0, 5.0);
        assert_eq!(v.length(), f64::sqrt(38.0));
    }

    #[test]
    fn should_add_correctly() {
        let v1 = Vec3::new(3.0, 2.0, 5.0);
        let v2 = Vec3::new(4.0, 6.0, 5.0);
        let v3 = v1 + v2;
        assert_eq!(
            v3,
            Vec3 {
                x: 7.0,
                y: 8.0,
                z: 10.0
            }
        );
    }

    #[test]
    fn should_sub_correctly() {
        let v1 = Vec3::new(3.0, 2.0, 5.0);
        let v2 = Vec3::new(4.0, 6.0, 5.0);
        let v3 = v1 - v2;
        assert_eq!(
            v3,
            Vec3 {
                x: -1.0,
                y: -4.0,
                z: 0.0
            }
        );
    }

    #[test]
    fn should_mul_vec_correctly() {
        let v1 = Vec3::new(3.0, 2.0, 5.0);
        let v2 = Vec3::new(4.0, 6.0, 5.0);
        let v3 = v1 * v2;
        assert_eq!(
            v3,
            Vec3 {
                x: 12.0,
                y: 12.0,
                z: 25.0
            }
        );
    }

    #[test]
    fn should_mul_correctly() {
        let v1 = Vec3::new(3.0, 2.0, 5.0);
        let v3 = v1 * 10.0;
        assert_eq!(
            v3,
            Vec3 {
                x: 30.0,
                y: 20.0,
                z: 50.0
            }
        );
    }

    #[test]
    fn should_div_correctly() {
        let v1 = Vec3::new(30.0, 20.0, 50.0);
        let v3 = v1 / 10.0;
        assert_eq!(
            v3,
            Vec3 {
                x: 3.0,
                y: 2.0,
                z: 5.0
            }
        );
    }

    #[test]
    fn should_dot_correctly() {
        let v1 = Vec3::new(3.0, 2.0, 5.0);
        let v2 = Vec3::new(4.0, 6.0, 5.0);
        let v3 = Vec3::dot(&v1, &v2);
        assert_eq!(v3, 49.0);
    }

    #[test]
    fn should_cross_correctly() {
        let v1 = Vec3::new(3.0, 2.0, 5.0);
        let v2 = Vec3::new(4.0, 6.0, 5.0);
        let v3 = v1.cross(&v2);
        assert_eq!(
            v3,
            Vec3 {
                x: -20.0,
                y: 5.0,
                z: 10.0
            }
        );
    }

    #[test]
    fn should_get_correct_unit_vector() {
        let v1 = Vec3::new(8.0, -3.0, 5.0);
        let v3 = Vec3::unit_vector(&v1);
        assert_eq!(
            v3,
            Vec3 {
                x: 0.8081220356417685,
                y: -0.30304576336566325,
                z: 0.5050762722761054
            }
        );
    }

    #[test]
    fn should_have_negative_trait() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v3 = -v1;
        assert_eq!(v3, Vec3::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn should_be_indexable() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let values = vec![1.0, 2.0, 3.0];
        for i in 0..3 {
            assert_eq!(Some(values[i]), v.at(i));
        }
    }

    #[test]
    fn should_not_be_indexable_when_out_of_bounds() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(None, v.at(3));
    }
}
