use std::num::Float;
use vec::{ Vec3, dot };
use ray::{ Ray, Inter };

pub trait Object {
    fn intersect(&self, ray: &Ray) -> Option<Inter>;
}

pub struct Objects<'a> {
    all: Vec<Box<Object + 'a>>,
}

impl<'a> Objects<'a> {
    pub fn new(all: Vec<Box<Object + 'a>>) -> Objects<'a> {
        Objects { all: all }
    }

    pub fn add(&mut self, object: Box<Object + 'a>) {
        self.all.push(object);
    }
}

impl<'a> Object for Objects<'a> {
    fn intersect(&self, ray: &Ray) -> Option<Inter> {
        let mut inter: Option<Inter> = None;
        for object in self.all.iter() {
            let cur_inter = object.intersect(&ray);
            if cur_inter.is_some() && (inter.is_none() || cur_inter.unwrap().dist < inter.unwrap().dist) {
                inter = cur_inter;
            }
        }
        inter
    }
}

#[allow(dead_code)]
pub struct Sphere {
    pos:    Vec3,
    radius: f64,
    color:  [f64; 3],
}

impl Sphere {
    #[allow(dead_code)]
    pub fn new(pos: Vec3, radius: f64, color: [f64; 3]) -> Sphere {
        Sphere { pos: pos, radius: radius, color: color }
    }
}

impl Object for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Inter> {
        let temporary = ray.pos - self.pos;
        let b = 2. * dot(ray.dir, temporary);
        let a = dot(ray.dir, ray.dir);
        let c = dot(temporary, temporary) - self.radius * self.radius;
        let disc = b * b - 4. * a * c;
        if disc < 0. {
            return None
        }
        let discriminent = disc.sqrt();
        let t1 = (-b + discriminent) / (2. * a);
        let t2 = (-b - discriminent) / (2. * a);
        if t1 <= 0. && t2 <= 0. {
            return None
        }
        let dist = if t2 <= 0. || t1 < t2 { t1 } else { t2 };
        let pos = ray.pos + ray.dir * dist;
        let normal = (pos - self.pos).normalize();
        Some(Inter::new(dist, pos, normal, self.color))
    }
}

#[allow(dead_code)]
pub struct Plane {
    pos:    Vec3,
    normal: Vec3,
    color:  [f64; 3],
}

impl Plane {
    #[allow(dead_code)]
    pub fn new(pos: Vec3, normal: Vec3, color: [f64; 3]) -> Plane {
        Plane { pos: pos, normal: normal.normalize(), color: color }
    }
}

impl Object for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Inter> {
        let dist = dot(self.pos - ray.pos, self.normal) / dot(ray.dir, self.normal);
        if dist < 0. {
            return None
        }
        let pos = ray.pos + ray.dir * dist;
        Some(Inter::new(dist, pos, self.normal, self.color))
    }
}

#[allow(dead_code)]
pub enum Dir {
    Left,   // -X
    Right,  // X
    Top,    // Y
    Bottom, // -Y
    Front,  // Z
    Back,   // -Z
}

#[allow(dead_code)]
pub struct AARect {
    pos:    Vec3,
    dir:    Dir,
    dim:    Vec3,
    normal: Vec3,
    color:  [f64; 3],
}

impl AARect {
    #[allow(dead_code)]
    pub fn new(pos: Vec3, dir: Dir, dim: Vec3, color: [f64; 3]) -> AARect {
        let normal = match dir {
            Dir::Left   => Vec3::new(-1., 0., 0.),
            Dir::Right  => Vec3::new(1., 0., 0.),
            Dir::Top    => Vec3::new(0., 1., 0.),
            Dir::Bottom => Vec3::new(0., -1., 0.),
            Dir::Front  => Vec3::new(0., 0., 1.),
            Dir::Back   => Vec3::new(0., 0., -1.),
        };
        AARect { pos: pos, dir: dir, dim: dim, normal: normal, color: color }
    }
}

impl Object for AARect {
    fn intersect(&self, ray: &Ray) -> Option<Inter> {
        // Plane intersection
        let dist = dot(self.pos - ray.pos, self.normal) / dot(ray.dir, self.normal);
        if dist < 0. {
            return None
        }
        let pos = ray.pos + ray.dir * dist;

        // AARect intersection
        let diff = pos - self.pos;
        if diff.x.abs() * 2. > self.dim.x || diff.y.abs() * 2. > self.dim.y || diff.z.abs() * 2. > self.dim.z {
            return None
        }

        Some(Inter::new(dist, pos, self.normal, self.color))
    }
}

#[allow(dead_code)]
pub struct AABox<'a> {
    faces: Objects<'a>,
}

impl<'a> AABox<'a> {
    #[allow(dead_code)]
    pub fn new(pos: Vec3, dim: Vec3, color: [f64; 3]) -> AABox<'a> {
        let left_pos = Vec3::new(pos.x - dim.x / 2., pos.y, pos.z);
        let right_pos = Vec3::new(pos.x + dim.x / 2., pos.y, pos.z);
        let top_pos = Vec3::new(pos.x, pos.y + dim.y / 2., pos.z);
        let bottom_pos = Vec3::new(pos.x, pos.y - dim.y / 2., pos.z);
        let front_pos = Vec3::new(pos.x, pos.y, pos.z + dim.z / 2.);
        let back_pos = Vec3::new(pos.x, pos.y, pos.z - dim.z / 2.);

        AABox { faces: Objects::new(vec![
            box AARect::new(left_pos, Dir::Left, dim, color),
            box AARect::new(right_pos, Dir::Right, dim, color),
            box AARect::new(top_pos, Dir::Top, dim, color),
            box AARect::new(bottom_pos, Dir::Bottom, dim, color),
            box AARect::new(front_pos, Dir::Front, dim, color),
            box AARect::new(back_pos, Dir::Back, dim, color),
        ]) }
    }
}

impl<'a> Object for AABox<'a> {
    fn intersect(&self, ray: &Ray) -> Option<Inter> {
        self.faces.intersect(ray)
    }
}
