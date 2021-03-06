use std::num::Float;
use std::old_io::fs::File;
use std::old_io::stdio;
use image::*;
use vec::{ Vec3, rotate, dot };
use ray::{ Ray, Inter };
use material::Color;
use object::{ Object, Objects };
use light::{ Light, Lights };

pub struct Picture {
    pub w:    u32,
    pub h:    u32,
    pub path: Path,
    bounce:   u32,
    sample:   u32,
}

impl Picture {
    pub fn new(w: u32, h: u32, path: &str, bounce: u32, sample: u32) -> Picture {
        Picture { w: w, h: h, path: Path::new(path), bounce: bounce, sample: sample }
    }

    // Picture a scene
    pub fn shot(&self, eye: &Eye, scene: &Scene, progress: bool) -> DynamicImage {
        // Initialize variables used to compute ray
        let w = (self.w * self.sample) as f64;
        let h = (self.h * self.sample) as f64;
        let dist = 100.;
        let screen_x = (eye.fov / 2.).tan() * dist;
        let screen_y = screen_x * h / w;
        let step = Vec3::new(screen_x / w, -screen_y / h, 0.);
        let start = Vec3::new(-screen_x / 2., screen_y / 2., -dist) + step / 2.;

        // Make ray
        let make_ray = |px, py, sx, sy| {
            let x = px * self.sample + sx;
            let y = py * self.sample + sy;
            let cur = Vec3::new(x as f64, y as f64, 0.);
            let mut dir = start + cur * step;
            dir = rotate(dir, eye.dir).normalize();
            Ray::new(eye.pos, dir)
        };

        // Create raw buffer of pixels
        let mut pixels = Vec::with_capacity((self.h * self.w) as usize);
        let to_u8 = 255. / (self.sample * self.sample) as f64;
        for py in 0..self.h {
            for px in 0..self.w {
                let color = (0..(self.sample * self.sample))
                    .map(|c| (c / self.sample, c % self.sample))
                    .map(|(sx, sy)| {
                        // Compute color
                        scene.raytrace(make_ray(px, py, sx, sy), 1. /* Air */, self.bounce)
                    })
                    .fold(Color::new(0., 0., 0.), |acc, item| acc + item) * to_u8;

                // Push pixel's colors to raw buffer
                pixels.push(color.r as u8);
                pixels.push(color.g as u8);
                pixels.push(color.b as u8);
            }

            // Show progress
            if progress {
                print!("\r{:03}%", (py + 1) * 100 / self.h);
                stdio::flush();
            }
        }

        // Show progress
        if progress {
            println!("");
        }

        // Create image from raw buffer
        let img_buf = ImageBuffer::from_raw(self.w, self.h, pixels).unwrap();
        ImageRgb8(img_buf)
    }

    // Write image to file
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
    ambient: f64,
    back:    Color,
}

impl<'a> Scene<'a> {
    pub fn new(objects: Objects<'a>, lights: Lights<'a>, ambient: f64, back: Color) -> Scene<'a> {
        Scene { objects: objects, lights: lights, ambient: ambient, back: back }
    }

    #[allow(dead_code)]
    pub fn add_object(&mut self, object: Box<Object + 'a>) {
        self.objects.add(object);
    }

    #[allow(dead_code)]
    pub fn add_light(&mut self, light: Box<Light + 'a>) {
        self.lights.add(light);
    }

    pub fn raytrace(&self, ray: Ray, refr_idx: f64, count: u32) -> Color {
        // Compute intersection
        let inter = self.objects.intersect(&ray);
        if inter.is_none() {
            return self.back;
        }

        // Compute lighting
        let mat = &inter.as_ref().unwrap().mat;
        let mut color = mat.color;
        let (spec, diff) = self.lights.bright(&ray, inter.as_ref().unwrap(), self);
        color = color * diff * mat.diff + Color::new(1., 1., 1.) * spec * mat.spec;

        // Compute refraction
        if mat.refr != 0. && count > 0 {
            color = color + self.refraction(ray.dir, refr_idx, inter.as_ref().unwrap(), count);
        }

        // Compute reflection
        if mat.refl != 0. && count > 0 {
            color = color + self.reflection(ray.dir, refr_idx, inter.as_ref().unwrap(), count);
        }

        color.normalize()
    }

    fn refraction(&self, ray_dir: Vec3, refr_idx: f64, inter: &Inter, count: u32) -> Color {
        let n = refr_idx / inter.mat.refr_idx;
        let c1 = -dot(inter.normal, ray_dir);
        let c2 = (1. - n * n * (1. - c1 * c1)).sqrt();
        let dir = (ray_dir * n + inter.normal * (n * c1 - c2)).normalize();
        let ray = Ray::new(inter.pos + dir * 0.00001, dir);

        self.raytrace(ray, inter.mat.refr_idx, count - 1) * inter.mat.refr
    }

    fn reflection(&self, ray_dir: Vec3, refr_idx: f64, inter: &Inter, count: u32) -> Color {
        let c1 = -dot(inter.normal, ray_dir);
        let dir = (ray_dir + inter.normal * 2. * c1).normalize();
        let ray = Ray::new(inter.pos + dir * 0.00001, dir);

        self.raytrace(ray, refr_idx, count - 1) * inter.mat.refl
    }

    pub fn shadow(&self, from: Vec3, to: Vec3) -> f64 {
        let dir = (to - from).normalize();
        let ray = Ray::new(from + dir * 0.00001, dir);

        // Compute intersection
        let inter = self.objects.intersect(&ray);
        if inter.is_none() || (to - inter.unwrap().pos).x * dir.x < 0. {
            return 1.;
        }
        self.ambient
    }
}
