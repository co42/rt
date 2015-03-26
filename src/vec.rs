use std::num::Float;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        Vec3::new(self.x / len, self.y / len, self.z / len)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

pub fn dot(left: Vec3, right: Vec3) -> f64 {
    left.x * right.x + left.y * right.y + left.z * right.z
}

pub fn cross(left: Vec3, right: Vec3) -> Vec3 {
    Vec3::new(left.y * right.z - left.z * right.y, left.z * right.x - left.x * right.z, left.x * right.z - left.y * right.x)
}

pub fn rotate(vec: Vec3, dir: Vec3) -> Vec3 {
    let mut res = vec;
    if dir.x != 0. {
        res = Vec3::new(
            res.x,
            res.y * dir.x.cos() + -res.z * dir.x.sin(),
            res.y * dir.x.sin() + res.z * dir.x.cos(),
        );
    }
    if dir.y != 0. {
        res = Vec3::new(
            res.x * dir.y.cos() + res.z * dir.y.sin(),
            res.y,
            -res.x * dir.y.sin() + res.z * dir.y.cos(),
        );
    }
    if dir.z != 0. {
        res = Vec3::new(
            res.x * dir.z.cos() + -res.y * dir.z.sin(),
            res.x * dir.z.sin() + res.y * dir.z.cos(),
            res.z,
        );
    }
    res
}
