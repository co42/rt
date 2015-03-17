use std::num::Float;
use vec::{ Vec3, dot };
use ray::{ Ray, Inter };
use material::Material;

pub trait Light {
    fn color(&self, mat: &mut Material, ray: &Ray, inter: Inter);
}

pub struct Lights<'a> {
    all: Vec<Box<Light + 'a>>,
}

impl<'a> Lights<'a> {
    pub fn new(all: Vec<Box<Light + 'a>>) -> Lights<'a> {
        Lights { all: all }
    }

    pub fn add(&mut self, light: Box<Light + 'a>) {
        self.all.push(light);
    }
}

impl<'a> Light for Lights<'a> {
    fn color(&self, mat: &mut Material, ray: &Ray, inter: Inter) {
        for light in self.all.iter() {
            light.color(mat, ray, inter);
        }
    }
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
    fn color(&self, mat: &mut Material, ray: &Ray, inter: Inter) {
        let l = (self.pos - inter.pos).normalize();
        let n = inter.normal;
        let r = (n * 2. * dot(l, n) - l).normalize();
        let v = (ray.pos - inter.pos).normalize();

        let diff = self.diff * mat.diff * dot(l, n).max(0.);
        let spec = self.spec * mat.spec * dot(r, v).max(0.).powi(self.shin);

        mat.color.r = mat.color.r * diff + spec;
        mat.color.g = mat.color.g * diff + spec;
        mat.color.b = mat.color.b * diff + spec;
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
    fn color(&self, mat: &mut Material, ray: &Ray, inter: Inter) {
        let l = self.dir * -1.;
        let n = inter.normal;
        let r = (n * 2. * dot(l, n) - l).normalize();
        let v = (ray.pos - inter.pos).normalize();

        let diff = self.diff * mat.diff * dot(l, n).max(0.);
        let spec = self.spec * mat.spec * dot(r, v).max(0.).powi(self.shin);

        mat.color.r = mat.color.r * diff + spec;
        mat.color.g = mat.color.g * diff + spec;
        mat.color.b = mat.color.b * diff + spec;
    }
}
