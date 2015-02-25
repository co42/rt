use std::num::Float;
use vec::{ Vec3, dot };
use ray::{ Ray, Inter };

pub trait Object {
    fn intersect(&self, ray: &Ray) -> Option<Inter>;
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
