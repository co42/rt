use vec::{ Vec3, dot };
use ray::Inter;

pub trait Light {
    fn diffuse(&self, inter: Inter) -> f64;
}

#[allow(dead_code)]
pub struct Bulb {
    pos: Vec3,
}

impl Bulb {
    #[allow(dead_code)]
    pub fn new(pos: Vec3) -> Bulb {
        Bulb { pos: pos }
    }
}

impl Light for Bulb {
    fn diffuse(&self, inter: Inter) -> f64 {
        let l1 = (self.pos - inter.pos).normalize();
        let l2 = dot(inter.normal, l1);
        if l2 < 0. {
            return 0.
        }
        l2
    }
}

#[allow(dead_code)]
pub struct Sun {
    dir: Vec3,
}

impl Sun {
    #[allow(dead_code)]
    pub fn new(dir: Vec3) -> Sun {
        Sun { dir: dir.normalize() }
    }
}

impl Light for Sun {
    fn diffuse(&self, inter: Inter) -> f64 {
        let l1 = self.dir * -1.;
        let l2 = dot(inter.normal, l1);
        if l2 < 0. {
            return 0.
        }
        l2
    }
}
