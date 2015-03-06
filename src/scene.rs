use std::num::Float;
use std::old_io::fs::File;
use image::*;
use vec::{ Vec3, rotate };
use ray::Ray;
use material::Color;
use object::{ Object, Objects };
use light::{ Light, Lights };

pub struct Picture {
    pub w:    u32,
    pub h:    u32,
    pub path: Path,
}

impl Picture {
    pub fn new(w: u32, h: u32, path: &str) -> Picture {
        Picture { w: w, h: h, path: Path::new(path) }
    }

    pub fn shot(&self, eye: &Eye, scene: &Scene, progress: bool) -> DynamicImage {
        // Initialize variables used to compute ray
        let dist = 100.;
        let screen_x = (eye.fov / 2.).tan() * dist;
        let screen_y = screen_x * self.h as f64 / self.w as f64;
        let step = Vec3::new(screen_x / self.w as f64, -screen_y / self.h as f64, 0.);
        let start = Vec3::new(-screen_x / 2., screen_y / 2., -dist) + step / 2.;

        // Create raw buffer of pixels
        let mut pixels = Vec::with_capacity((self.h * self.w) as usize);
        for y in 0..self.h {
            for x in 0..self.w {
                // Create ray
                let cur = Vec3::new(x as f64, y as f64, 0.);
                let mut dir = start + cur * step;
                dir = rotate(dir, eye.dir).normalize();
                let ray = Ray::new(eye.pos, dir);

                // Compute and push pixel's colors to raw buffer
                let color = scene.raytrace(ray);
                let to_u8 = |c| (c * 255.) as u8; // Convert color from 0..1f64 to 0..255u8
                pixels.push(to_u8(color.r));
                pixels.push(to_u8(color.g));
                pixels.push(to_u8(color.b));
            }
            if progress {
                println!("\r{:03}%", (y + 1) * 100 / self.h);
            }
        }

        // Create image from raw buffer
        let img_buf = ImageBuffer::from_raw(self.w, self.h, pixels).unwrap();
        ImageRgb8(img_buf)
    }

    pub fn save(&self, img: &DynamicImage) {
        let mut out = File::create(&self.path).unwrap();
        let _ = img.save(&mut out, PNG);
    }
}

pub struct Eye {
    pos: Vec3,
    dir: Vec3,
    fov: f64,
}

impl Eye {
    pub fn new(pos: Vec3, dir: Vec3, fov: f64) -> Eye {
        Eye { pos: pos, dir: dir, fov: fov }
    }
}

pub struct Scene<'a> {
    objects: Objects<'a>,
    lights:  Lights<'a>,
}

impl<'a> Scene<'a> {
    pub fn new(objects: Objects<'a>, lights: Lights<'a>) -> Scene<'a> {
        Scene { objects: objects, lights: lights }
    }

    #[allow(dead_code)]
    pub fn empty() -> Scene<'a> {
        Scene { objects: Objects::new(vec![]), lights: Lights::new(vec![]) }
    }

    #[allow(dead_code)]
    pub fn add_object(&mut self, object: Box<Object + 'a>) {
        self.objects.add(object);
    }

    #[allow(dead_code)]
    pub fn add_light(&mut self, light: Box<Light + 'a>) {
        self.lights.add(light);
    }

    pub fn raytrace(&self, ray: Ray) -> Color {
        // Compute intersection
        let inter = self.objects.intersect(&ray);
        if inter.is_none() {
            return Color::new(0., 0., 0.);
        }

        // Compute lighting
        let mut mat = inter.unwrap().mat;
        self.lights.color(&mut mat, &ray, inter.unwrap());
        mat.color
    }
}
