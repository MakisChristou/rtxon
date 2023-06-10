use std::ops;

// mod vec3;
use crate::utils::random_double;
use crate::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn default() -> Self {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn random(range: Option<(f64, f64)>) -> Self {
        match range {
            None => Color::new(
                random_double(None),
                random_double(None),
                random_double(None),
            ),
            Some(range) => Color::new(
                random_double(Some(range)),
                random_double(Some(range)),
                random_double(Some(range)),
            ),
        }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, _rhs: f64) -> Color {
        Color::new(self.r * _rhs, self.g * _rhs, self.b * _rhs)
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Color {
        Color::new(self.r * _rhs.r, self.g * _rhs.g, self.b * _rhs.b)
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, _rhs: Color) -> Color {
        Color::new(self.r + _rhs.r, self.g + _rhs.g, self.b + _rhs.b)
    }
}

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Color {
        Color::new(v.x, v.y, v.z)
    }
}

#[cfg(test)]

mod tests {
    use crate::utils::color::Color;

    #[test]
    fn should_multiply_f64_correctly() {
        let c1 = Color::new(10.0, 10.0, 10.0);
        let c3 = c1 * 0.5;
        assert_eq!(c3, Color::new(5.0, 5.0, 5.0));
    }

    #[test]
    fn should_multiply_color_correctly() {
        let c1 = Color::new(10.0, 10.0, 10.0);
        let c2 = Color::new(0.5, 0.5, 0.5);
        let c3 = c1 * c2;
        assert_eq!(c3, Color::new(5.0, 5.0, 5.0));
    }
}
