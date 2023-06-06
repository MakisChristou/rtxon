use std::ops;

#[derive(Debug, PartialEq)]
pub struct vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,   
}

impl vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self{
        vec3{x,y,z}
    }

    pub fn from_one(a: f64) -> Self{
        vec3{x: a, y: a, z: a}
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl ops::Add<vec3> for vec3 {
    type Output = vec3;
    fn add(self, _rhs: vec3) -> vec3 {
        vec3::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z) 
    }
}

impl ops::Sub<vec3> for vec3 {
    type Output = vec3;
    fn sub(self, _rhs: vec3) -> vec3 {
        vec3::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z) 
    }
}

impl ops::Mul<vec3> for vec3 {
    type Output = vec3;

    fn mul(self, _rhs: vec3) -> vec3{
        vec3::new(self.x * _rhs.x, self.y * _rhs.y, self.z * _rhs.z) 
    }
}

impl ops::Mul<f64> for vec3 {
    type Output = vec3;

    fn mul(self, _rhs: f64) -> vec3{
        vec3::new(self.x * _rhs, self.y * _rhs, self.z * _rhs) 
    }
}


#[cfg(test)]
mod tests{
    use crate::vec3::vec3;


    #[test]
    fn should_instantiate_vector() {
        let v = vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v, vec3{x: 1.0, y: 2.0, z: 3.0});
    }

    #[test]
    fn should_instantiate_from_one() {
        let v = vec3::from_one(1.0);
        assert_eq!(v, vec3{x: 1.0, y: 1.0, z: 1.0});
    }

    #[test]
    fn should_have_correct_length_squared(){
        let v = vec3::new(1.0, 1.0, 5.0);
        assert_eq!(v.length_squared(), 27.0);
    }

    #[test]
    fn should_have_correct_length(){
        let v = vec3::new(3.0, 2.0, 5.0);
        assert_eq!(v.length(), f64::sqrt(38.0));
    }

    #[test]
    fn should_add_correctly(){
        let v1 = vec3::new(3.0, 2.0, 5.0);
        let v2 = vec3::new(4.0, 6.0, 5.0);
        let v3 = v1 + v2;
        assert_eq!(v3, vec3{x: 7.0, y: 8.0, z: 10.0});
    }

    #[test]
    fn should_sub_correctly(){
        let v1 = vec3::new(3.0, 2.0, 5.0);
        let v2 = vec3::new(4.0, 6.0, 5.0);
        let v3 = v1 - v2;
        assert_eq!(v3, vec3{x: -1.0, y: -4.0, z: 0.0});
    }

    #[test]
    fn should_mul_vec_correctly(){
        let v1 = vec3::new(3.0, 2.0, 5.0);
        let v2 = vec3::new(4.0, 6.0, 5.0);
        let v3 = v1 * v2;
        assert_eq!(v3, vec3{x: 12.0, y: 12.0, z: 25.0});
    }

    #[test]
    fn should_mul_correctly(){
        let v1 = vec3::new(3.0, 2.0, 5.0);
        let v3 = v1 * 10.0;
        assert_eq!(v3, vec3{x: 30.0, y: 20.0, z: 50.0});
    }

}