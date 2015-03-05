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
}

#[derive(Copy)]
pub struct Material {
    pub color: Color,
    pub spec: f64, // Specular
    pub diff: f64, // Diffuse
}

impl Material {
    pub fn new(color: Color, spec: f64, diff: f64) -> Material {
        Material { color: color, spec: spec, diff: diff }
    }
}
