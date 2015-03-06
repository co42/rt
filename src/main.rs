#![feature(core)]
#![feature(box_syntax)]
#![feature(old_path)]
#![feature(old_io)]

extern crate num;
extern crate "rustc-serialize" as serialize;
extern crate image;

mod vec;
mod ray;
mod material;
mod object;
mod light;
mod scene;
mod frac;
mod config;

fn main() {
    // Load eye and scene
    let input = std::old_io::stdio::stdin().read_to_string().unwrap();
    let (eye, scene, picture) = config::load(input.as_slice());

    // Compute and save image
    let img = picture.shot(&eye, &scene, true);
    picture.save(&img);
}
