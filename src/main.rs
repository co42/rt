#![feature(core)]
#![feature(box_syntax)]
#![feature(convert)]

extern crate num;
extern crate rustc_serialize;
extern crate image;

use std::io::Read;

mod vec;
mod ray;
mod material;
mod object;
mod light;
mod scene;
mod config;

fn main() {
    // Load eye and scene
    let mut input = String::new();
    let _ = std::io::stdin().read_to_string(&mut input);
    let (eye, scene, picture) = config::load(input.as_ref());

    // Compute and save image
    let img = picture.shot(&eye, &scene, true);
    picture.save(&img);
}
