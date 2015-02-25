use vec::Vec3;

pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(pos: Vec3, dir: Vec3) -> Ray {
        Ray { pos: pos, dir: dir }
    }
}

#[derive(Copy)]
pub struct Inter {
    pub dist:   f64,
    pub pos:    Vec3,
    pub normal: Vec3,
    pub color:  [f64; 3],
}

impl Inter {
    pub fn new(dist: f64, pos: Vec3, normal: Vec3, color: [f64; 3]) -> Inter {
        Inter { dist: dist, pos: pos, normal: normal, color: color }
    }
}
