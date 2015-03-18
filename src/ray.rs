use std::rc::Rc;
use vec::Vec3;
use material::Material;

pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(pos: Vec3, dir: Vec3) -> Ray {
        Ray { pos: pos, dir: dir }
    }
}

pub struct Inter {
    pub dist:   f64,
    pub pos:    Vec3,
    pub normal: Vec3,
    pub mat:    Rc<Material>,
}

impl Inter {
    pub fn new(dist: f64, pos: Vec3, normal: Vec3, mat: Rc<Material>) -> Inter {
        Inter { dist: dist, pos: pos, normal: normal, mat: mat }
    }
}
