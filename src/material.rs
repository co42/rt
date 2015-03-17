use std::ops::{Add, Mul};
use std::num::Float;

#[derive(Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r: r, g: g, b: b }
    }

    pub fn normalize(&self) -> Color {
        Color::new(self.r.min(1.), self.g.min(1.), self.b.min(1.))
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

#[derive(Copy)]
pub struct Material {
    pub color:     Color,
    pub spec:      f64,  // Specular
    pub diff:      f64,  // Diffuse
    pub refr:      f64,  // Refraction
    pub refr_idx:  f64,  // Refractive indice
    pub refl:      f64,  // Reflection
}

impl Material {
    pub fn new(color: Color, spec: f64, diff: f64, refr: f64, refr_idx: f64, refl: f64) -> Material {
        Material { color: color, spec: spec, diff: diff, refr: refr, refr_idx: refr_idx, refl: refl }
    }
}
