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

#[cfg(test)]
mod tests{
    use crate::vec3::vec3;


    #[test]
    fn should_instantiate_vector() {
        let v = vec3::new(1.0, 2.0, 3.0);
        assert!(v.x == 1.0 && v.y == 2.0 && v.z == 3.0);
    }

    #[test]
    fn should_have_correct_length_squared(){
        let v = vec3::new(1.0, 1.0, 5.0);
        assert!(v.length_squared() == 27.0);
    }

    #[test]
    fn should_have_correct_length(){
        let v = vec3::new(3.0, 2.0, 5.0);
        assert!(v.length() == f64::sqrt(38.0));
    }

}