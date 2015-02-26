#![feature(box_syntax)]
#![feature(old_path)]
#![feature(old_io)]
#![feature(collections)]

extern crate num;
extern crate image;

use std::num::Float;
use std::old_io::fs::File;
use image::*;
use vec::*;
use ray::*;
use object::*;
use light::*;

mod vec;
mod ray;
mod object;
mod light;
mod frac;

struct Eye {
    pos: Vec3,
    dir: Vec3,
    fov: f64,
}

impl Eye {
    fn new(pos: Vec3, dir: Vec3, fov: f64) -> Eye {
        Eye { pos: pos, dir: dir, fov: fov }
    }

    fn picture<F>(&self, w: u32, h: u32, f: F) -> DynamicImage
        where F : Fn(Ray) -> [u8; 3] {

        // Initialize variables used to compute ray
        let dist = 100.;
        let screen_x = (self.fov / 2.).tan() * dist;
        let screen_y = screen_x * h as f64 / w as f64;
        let step = Vec3::new(screen_x / w as f64, -screen_y / h as f64, 0.);
        let start = Vec3::new(-screen_x / 2., screen_y / 2., -dist) + step / 2.;

        // Create raw buffer of pixels
        let mut pixels = Vec::with_capacity((h * w) as usize);
        for y in 0..h {
            for x in 0..w {
                // Create ray
                let cur = Vec3::new(x as f64, y as f64, 0.);
                let mut dir = start + cur * step;
                dir = rotate(dir, self.dir).normalize();
                let ray = Ray::new(self.pos, dir);

                // Compute and push pixel's color to raw buffer
                pixels.push_all(&f(ray));
            }
        }

        // Create image from raw buffer
        let img_buf = ImageBuffer::from_raw(w, h, pixels).unwrap();
        ImageRgb8(img_buf)
    }
}

struct Scene<'a> {
    objects: Objects<'a>,
    lights:  Vec<Box<Light + 'a>>,
}

impl<'a> Scene<'a> {
    fn new() -> Scene<'a> {
        Scene { objects: Objects::new(vec![]), lights: vec![] }
    }

    fn add_object(&mut self, object: Box<Object + 'a>) {
        self.objects.add(object);
    }

    fn add_light(&mut self, light: Box<Light + 'a>) {
        self.lights.push(light);
    }

    fn raytrace(&self, ray: Ray) -> [f64; 3] {
        // Compute intersection
        let inter = self.objects.intersect(&ray);
        if inter.is_none() {
            return [0., 0., 0.];
        }

        // Compute lighting
        let mut color = inter.unwrap().color;
        for light in self.lights.iter() {
            color = light.color(color, &ray, inter.unwrap());
        }
        color
    }
}

fn main() {
    // Initialize scene
    let mut scene = Scene::new();
    scene.add_object(box Plane::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 1.), [0., 1., 0.]));
    scene.add_object(box Sphere::new(Vec3::new(40., 0., 0.), 20., [0., 0., 1.]));
    scene.add_object(box AABox::new(Vec3::new(-40., 0., 0.), Vec3::new(40., 20., 10.), [0., 0., 1.]));
    scene.add_light(box Bulb::new(Vec3::new(0., 10., 100.), 1.5, 20, 0.7));

    // Fill image
    let eye = Eye::new(Vec3::new(0., 0., 100.), Vec3::new(0., 0., 0.), 2.1 /* 120° */);
    let img = eye.picture(1280, 720, |ray| {
        let color = scene.raytrace(ray);
        // Convert color from 0..1f64 to 0..255u8
        let to_u8 = |c| (c * 255.) as u8;
        [to_u8(color[0]), to_u8(color[1]), to_u8(color[2])]
    });

    // Save image
    let mut out = File::create(&Path::new("image/image.png")).unwrap();
    let _ = img.save(&mut out, PNG);
}
