use std::ops;

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, _rhs: f64) -> Color {
        Color::new(self.r * _rhs, self.g * _rhs, self.b * _rhs)
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, _rhs: Color) -> Color {
        Color::new(self.r + _rhs.r, self.g + _rhs.g, self.b + _rhs.b)
    }
}
