use std::num::Float;
use vec::{ Vec3, dot };
use ray::{ Ray, Inter };

pub trait Light {
    fn color(&self, color: [f64; 3], ray: &Ray, inter: Inter) -> [f64; 3];
}

#[allow(dead_code)]
pub struct Bulb {
    pos:  Vec3,
    spec: f64, // Specular
    shin: i32, // Shininess
    diff: f64, // Diffuse
}

impl Bulb {
    #[allow(dead_code)]
    pub fn new(pos: Vec3, spec: f64, shin: i32, diff: f64) -> Bulb {
        Bulb { pos: pos, spec: spec, shin: shin, diff: diff }
    }
}

impl Light for Bulb {
    fn color(&self, color: [f64; 3], ray: &Ray, inter: Inter) -> [f64; 3] {
        let l = (self.pos - inter.pos).normalize();
        let n = inter.normal;
        let r = (n * 2. * dot(l, n) - l).normalize();
        let v = (ray.pos - inter.pos).normalize();

        let diff = self.diff * dot(l, n).max(0.);
        let spec = self.spec * dot(r, v).max(0.).powi(self.shin);

        let r = (color[0] * diff + spec).min(1.);
        let g = (color[1] * diff + spec).min(1.);
        let b = (color[2] * diff + spec).min(1.);

        [r, g, b]
    }
}

#[allow(dead_code)]
pub struct Sun {
    dir:  Vec3,
    spec: f64, // Specular
    shin: i32, // Shininess
    diff: f64, // Diffuse
}

impl Sun {
    #[allow(dead_code)]
    pub fn new(dir: Vec3, spec: f64, shin: i32, diff: f64) -> Sun {
        Sun { dir: dir.normalize(), spec: spec, shin: shin, diff: diff }
    }
}

impl Light for Sun {
    fn color(&self, color: [f64; 3], ray: &Ray, inter: Inter) -> [f64; 3] {
        let l = self.dir * -1.;
        let n = inter.normal;
        let r = (n * 2. * dot(l, n) - l).normalize();
        let v = (ray.pos - inter.pos).normalize();

        let diff = self.diff * dot(l, n).max(0.);
        let spec = self.spec * dot(r, v).max(0.).powi(self.shin);

        let r = (color[0] * diff + spec).min(1.);
        let g = (color[1] * diff + spec).min(1.);
        let b = (color[2] * diff + spec).min(1.);

        [r, g, b]
    }
}
