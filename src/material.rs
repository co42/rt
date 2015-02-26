#[derive(Copy)]
pub struct Material {
    pub color: [f64; 3],
    pub spec: f64, // Specular
    pub diff: f64, // Diffuse
}

impl Material {
    pub fn new(color: [f64; 3], spec: f64, diff: f64) -> Material {
        Material { color: color, spec: spec, diff: diff }
    }
}
