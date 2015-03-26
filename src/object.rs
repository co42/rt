use std::num::Float;
use std::f64::consts::PI;
use std::cmp::partial_max;
use std::rc::Rc;
use vec::{ Vec3, dot, cross, rotate };
use ray::{ Ray, Inter };
use material::{ Color, Material };

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
            if cur_inter.is_some() && (inter.is_none() || cur_inter.as_ref().unwrap().dist < inter.as_ref().unwrap().dist) {
                inter = cur_inter;
            }
        }
        inter
    }
}

#[allow(dead_code)]
pub struct Rotate<'a> {
    pos:    Vec3,
    dir:    Vec3,
    object: Box<Object + 'a>,
}

impl<'a> Rotate<'a> {
    #[allow(dead_code)]
    pub fn new(pos: Vec3, dir: Vec3, object: Box<Object>) -> Rotate<'a> {
        Rotate { pos: pos, dir: dir, object: object }
    }
}

impl<'a> Object for Rotate<'a> {
    fn intersect(&self, ray: &Ray) -> Option<Inter> {
        let rot_ray = Ray::new(
            rotate(ray.pos - self.pos, self.dir) + self.pos,
            rotate(ray.dir, self.dir),
        );
        match self.object.intersect(&rot_ray) {
            Some(inter) => {
                Some(Inter::new(
                    inter.dist,
                    rotate(inter.pos - self.pos, self.dir * -1.) + self.pos,
                    rotate(inter.normal, self.dir * -1.),
                    inter.mat,
                ))
            },
            None        => None,
        }
    }
}

#[allow(dead_code)]
pub struct Sphere {
    pos:    Vec3,
    radius: f64,
    mat:    Rc<Material>,
}

impl Sphere {
    #[allow(dead_code)]
    pub fn new(pos: Vec3, radius: f64, mat: Rc<Material>) -> Sphere {
        Sphere { pos: pos, radius: radius, mat: mat }
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

        let normal = if (ray.pos - self.pos).length() > self.radius {
            (pos - self.pos).normalize()
        } else {
            (pos - self.pos).normalize() * -1.
        };
        Some(Inter::new(dist, pos, normal, self.mat.clone()))
    }
}

#[allow(dead_code)]
pub struct Plane {
    pos:    Vec3,
    normal: Vec3,
    mat:    Rc<Material>,
}

impl Plane {
    #[allow(dead_code)]
    pub fn new(pos: Vec3, normal: Vec3, mat: Rc<Material>) -> Plane {
        Plane { pos: pos, normal: normal.normalize(), mat: mat }
    }
}

impl Object for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Inter> {
        let dist = dot(self.pos - ray.pos, self.normal) / dot(ray.dir, self.normal);
        if dist < 0. {
            return None
        }
        let pos = ray.pos + ray.dir * dist;
        Some(Inter::new(dist, pos, self.normal, self.mat.clone()))
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
    mat:    Rc<Material>,
}

impl AARect {
    #[allow(dead_code)]
    pub fn new(pos: Vec3, dir: Dir, dim: Vec3, mat: Rc<Material>) -> AARect {
        let normal = match dir {
            Dir::Left   => Vec3::new(-1., 0., 0.),
            Dir::Right  => Vec3::new(1., 0., 0.),
            Dir::Top    => Vec3::new(0., 1., 0.),
            Dir::Bottom => Vec3::new(0., -1., 0.),
            Dir::Front  => Vec3::new(0., 0., 1.),
            Dir::Back   => Vec3::new(0., 0., -1.),
        };
        AARect { pos: pos, dir: dir, dim: dim, normal: normal, mat: mat }
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

        Some(Inter::new(dist, pos, self.normal, self.mat.clone()))
    }
}

#[allow(dead_code)]
pub struct AABox<'a> {
    faces: Objects<'a>,
}

impl<'a> AABox<'a> {
    #[allow(dead_code)]
    pub fn new(pos: Vec3, dim: Vec3, mat: Rc<Material>, skybox: bool) -> AABox<'a> {
        let sign = if skybox { -1. } else { 1. };

        let left_pos = Vec3::new(pos.x - dim.x / 2. * sign, pos.y, pos.z);
        let right_pos = Vec3::new(pos.x + dim.x / 2. * sign, pos.y, pos.z);
        let top_pos = Vec3::new(pos.x, pos.y + dim.y / 2. * sign, pos.z);
        let bottom_pos = Vec3::new(pos.x, pos.y - dim.y / 2. * sign, pos.z);
        let front_pos = Vec3::new(pos.x, pos.y, pos.z + dim.z / 2. * sign);
        let back_pos = Vec3::new(pos.x, pos.y, pos.z - dim.z / 2. * sign);

        AABox { faces: Objects::new(vec![
            box AARect::new(left_pos, Dir::Left, dim, mat.clone()),
            box AARect::new(right_pos, Dir::Right, dim, mat.clone()),
            box AARect::new(top_pos, Dir::Top, dim, mat.clone()),
            box AARect::new(bottom_pos, Dir::Bottom, dim, mat.clone()),
            box AARect::new(front_pos, Dir::Front, dim, mat.clone()),
            box AARect::new(back_pos, Dir::Back, dim, mat.clone()),
        ]) }
    }
}

impl<'a> Object for AABox<'a> {
    fn intersect(&self, ray: &Ray) -> Option<Inter> {
        self.faces.intersect(ray)
    }
}

#[allow(dead_code)]
pub struct AAHexa<'a> {
    faces: Objects<'a>,
}

impl<'a> AAHexa<'a> {
    #[allow(dead_code)]
    pub fn new(pos: Vec3, x: f64, y: f64, mat: Rc<Material>) -> AAHexa<'a> {
        let z = (PI / 6.).tan() * x;
        let dim = Vec3::new(x, y, z);

        AAHexa { faces: Objects::new(vec![
            box AABox::new(pos, dim, mat.clone(), false),
            box Rotate::new(pos, Vec3::new(0., PI / 3., 0.), box AABox::new(pos, dim, mat.clone(), false)),
            box Rotate::new(pos, Vec3::new(0., 2. * PI / 3., 0.), box AABox::new(pos, dim, mat.clone(), false)),
        ]) }
    }
}

impl<'a> Object for AAHexa<'a> {
    fn intersect(&self, ray: &Ray) -> Option<Inter> {
        self.faces.intersect(ray)
    }
}

#[allow(dead_code)]
pub struct HeightMap<'a> {
    pos:   Vec3,
    start: Vec3,
    ratio: f64,
    w:     usize,
    h:     usize,
    data:  Vec<HMData>,
    mat:   Rc<Material>,
    aabb:  AABox<'a>,
}

impl<'a> HeightMap<'a> {
    pub fn new(pos: Vec3, ratio: f64, w: usize, h: usize, data: Vec<HMData>, mat: Rc<Material>) -> HeightMap<'a> {
        let maxh = data.iter().fold(0., |acc, ref item| partial_max(acc, item.h).unwrap());
        let dim = Vec3::new((w - 1) as f64 * ratio, maxh * ratio, (h - 1) as f64 * ratio);
        let start = pos - Vec3::new(dim.x, 0., dim.z) / 2.;

        let aapos = start + dim / 2.;
        let aabb = AABox::new(aapos, dim, mat.clone(), false);

        HeightMap { pos: pos, start: start, ratio: ratio, w: w, h: h, data: data, mat: mat, aabb: aabb }
    }

    fn data_at(&self, cur: Vec3) -> Option<(HMData, Vec3)> {
        // Projection of the ray's position to the height map : [0, w], [0, h]
        let pos = (cur - self.start) / self.ratio;
        let x = pos.x;
        let y = pos.z;
        if x < 0. || x >= (self.w - 1) as f64 || y < 0. || y >= (self.h - 1) as f64 {
            return None
        }

        // Projection of the ray's position to a square [0, 1], [0, 1]
        let dx = x.fract();
        let dy = y.fract();

        // Weight of each data (corner)
        let length = |x: f64, y: f64| (x * x + y * y).sqrt();
        let r00 = 1. - length(dx, dy).min(1.);
        let r01 = 1. - length(dx, 1. - dy).min(1.);
        let r10 = 1. - length(1. - dx, dy).min(1.);
        let r11 = 1. - length(1. - dx, 1. - dy).min(1.);
        let rtotal = r00 + r01 + r10 + r11;

        // Square data (corners)
        let ref data00 = self.data[y as usize * self.w + x as usize];
        let ref data01 = self.data[(y + 1.) as usize * self.w + x as usize];
        let ref data10 = self.data[y as usize * self.w + (x + 1.) as usize];
        let ref data11 = self.data[(y + 1.) as usize * self.w + (x + 1.) as usize];

        // Corners' normals
        let v00_01 = Vec3::new(0., data01.h - data00.h, 1.);
        let v00_10 = Vec3::new(1., data10.h - data00.h, 0.);
        let n00 = cross(v00_01, v00_10).normalize();

        let v10_11 = Vec3::new(0., data11.h - data10.h, 1.);
        let v10_00 = Vec3::new(1., data00.h - data10.h, 0.);
        let n10 = cross(v10_11, v10_00).normalize();

        let v01_00 = Vec3::new(0., data00.h - data01.h, 1.);
        let v01_11 = Vec3::new(1., data11.h - data01.h, 0.);
        let n01 = cross(v01_00, v01_11).normalize();

        let v11_10 = Vec3::new(0., data10.h - data11.h, 1.);
        let v11_01 = Vec3::new(1., data01.h - data11.h, 0.);
        let n11 = cross(v11_10, v11_01).normalize();

        // Result ...
        let h = (data00.h * r00 + data01.h * r01 + data10.h * r10 + data11.h * r11) / rtotal;
        let color = (data00.color * r00 + data01.color * r01 + data10.color * r10 + data11.color * r11) / rtotal;
        let normal = (n00 * r00 + n01 * r01 + n10 * r10 + n11 * r11) / rtotal;

        Some((HMData::new(h, color), normal))
    }
}

impl<'a> Object for HeightMap<'a> {
    fn intersect(&self, ray: &Ray) -> Option<Inter> {
        // Test interction with the AABB
        let mut cur;
        match self.aabb.intersect(ray) {
            Some(ref inter) => cur = inter.pos,
            None            => return None,
        }

        // Test intersection with the height map (ray marching)
        let mut dist = (ray.pos - cur).length();
        for _ in 0..100 {
            match self.data_at(cur) {
                Some((ref data, normal)) => {
                    // Test intersection
                    let diff = data.h - cur.y;
                    if diff.abs() < 0.00001 {
                        let mat = Rc::new(Material::new(data.color, self.mat.spec, self.mat.diff, self.mat.refr, self.mat.refr_idx, self.mat.refl));
                        return Some(Inter::new(dist, cur, normal, mat));
                    }

                    // Compute next move
                    let step = diff / ray.dir.y;
                    dist += step;

                    // Intersection is behind
                    if dist < 0. {
                        return None
                    }

                    // Move closer to the height map
                    cur = cur + ray.dir * step;
                },
                // Height map missed
                None => {
                    return None;
                }
            }
        }

        None
    }
}

pub struct HMData {
    h:     f64,
    color: Color,
}

impl HMData {
    pub fn new(h: f64, color: Color) -> HMData {
        HMData { h: h, color: color }
    }
}
