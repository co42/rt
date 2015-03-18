use std::num::Float;
use vec::{ Vec3, dot };
use ray::{ Ray, Inter };
use scene::Scene;

pub trait Light {
    fn bright(&self, ray: &Ray, inter: &Inter, scene: &Scene) -> (f64, f64);
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

    fn bright_helper(light_pos: Vec3, shin: i32, ray: &Ray, inter: &Inter, scene: &Scene) -> (f64, f64) {
        let l = (light_pos - inter.pos).normalize();
        let r = (inter.normal * 2. * dot(l, inter.normal) - l).normalize();
        let v = (ray.pos - inter.pos).normalize();

        let s = scene.shadow(inter.pos, light_pos);
        let diff = s * dot(l, inter.normal).max(0.);
        let spec = s * dot(r, v).max(0.).powi(shin);

        (spec, diff)
    }
}

impl<'a> Light for Lights<'a> {
    fn bright(&self, ray: &Ray, inter: &Inter, scene: &Scene) -> (f64, f64) {
        self.all.iter()
            .map(|l| l.bright(ray, inter, scene))
            .fold((0., 0.), |acc, item| (acc.0 + item.0, acc.1 + item.1))
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
    fn bright(&self, ray: &Ray, inter: &Inter, scene: &Scene) -> (f64, f64) {
        let (spec, diff) = Lights::bright_helper(self.pos, self.shin, ray, inter, scene);
        (spec * self.spec, diff * self.diff)
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
    fn bright(&self, ray: &Ray, inter: &Inter, scene: &Scene) -> (f64, f64) {
        let pos = self.dir * -1000000.;
        let (spec, diff) = Lights::bright_helper(pos, self.shin, ray, inter, scene);
        (spec * self.spec, diff * self.diff)
    }
}
