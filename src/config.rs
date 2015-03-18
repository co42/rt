use std::rc::Rc;
use serialize::json::Json;
use vec::Vec3;
use material::{ Color, Material };
use object::{ Object, Objects, Rotate, Sphere, Plane, Dir, AARect, AABox, AAHexa };
use light::{ Light, Lights, Bulb, Sun };
use scene::{ Picture, Eye, Scene };

pub fn load(input: &str) -> (Eye, Scene, Picture) {
    let root = Json::from_str(input).unwrap();
    (load_eye(&root, "eye"), load_scene(&root, "scene"), load_picture(&root, "picture"))
}

// Picture
fn load_picture(root: &Json, key: &str) -> Picture {
    let obj = root.find(key).unwrap();
    Picture::new(
        load_u32(obj, "w"),
        load_u32(obj, "h"),
        load_str(obj, "path"),
        load_u32_or(obj, "bounce", 5),
        load_u32_or(obj, "sample", 1),
    )
}

// Eye
fn load_eye(root: &Json, key: &str) -> Eye {
    let obj = root.find(key).unwrap();
    Eye::new(
        load_vec3(obj, "pos"),
        load_vec3(obj, "dir"),
        load_f64(obj, "fov"),
    )
}

// Scene
fn load_scene<'a>(root: &Json, key: &str) -> Scene<'a> {
    let obj = root.find(key).unwrap();
    Scene::new(
        load_objects(obj, "objects"),
        load_lights(obj, "lights"),
        load_f64_or(obj, "ambient", 0.2),
        load_color_or(obj, "back", Color::new(0.39, 0.8, 0.92)),
    )
}

// Lights
fn load_lights<'a>(root: &Json, key: &str) -> Lights<'a> {
    let array = root.find(key).unwrap().as_array().unwrap();
    let all = array.iter().map(|obj| {
        let key = obj.as_object().unwrap().keys().next().unwrap();
        match key.as_slice() {
            "bulb" => box load_bulb(obj, key) as Box<Light>,
            "sun"  => box load_sun(obj, key) as Box<Light>,
            _      => panic!("Not a light"),
        }
    }).collect();
    Lights::new(all)
}

// Sun
fn load_sun(root: &Json, key: &str) -> Sun {
    let obj = root.find(key).unwrap();
    Sun::new(
        load_vec3(obj, "dir"),
        load_f64(obj, "spec"),
        load_i32(obj, "shin"),
        load_f64(obj, "diff"),
    )
}

// Bulb
fn load_bulb(root: &Json, key: &str) -> Bulb {
    let obj = root.find(key).unwrap();
    Bulb::new(
        load_vec3(obj, "pos"),
        load_f64(obj, "spec"),
        load_i32(obj, "shin"),
        load_f64(obj, "diff"),
    )
}

// Objects
fn load_objects<'a>(root: &Json, key: &str) -> Objects<'a> {
    let array = root.find(key).unwrap().as_array().unwrap();
    let all = array.iter().map(|obj| load_object(obj)).collect();
    Objects::new(all)
}

// Object
fn load_object(root: &Json) -> Box<Object> {
    let key = root.as_object().unwrap().keys().next().unwrap();
    match key.as_slice() {
        "rotate" => box load_rotate(root, key) as Box<Object>,
        "sphere" => box load_sphere(root, key) as Box<Object>,
        "plane"  => box load_plane(root, key) as Box<Object>,
        "aarect" => box load_aarect(root, key) as Box<Object>,
        "aabox"  => box load_aabox(root, key) as Box<Object>,
        "aahexa" => box load_aahexa(root, key) as Box<Object>,
        _        => panic!("Not an object"),
    }
}

// Rotate
fn load_rotate<'a>(root: &Json, key: &str) -> Rotate<'a> {
    let obj = root.find(key).unwrap();
    Rotate::new(
        load_vec3(obj, "pos"),
        load_vec3(obj, "dir"),
        load_object(obj.find("object").unwrap()),
    )
}

// AAHexa
fn load_aahexa<'a>(root: &Json, key: &str) -> AAHexa<'a> {
    let obj = root.find(key).unwrap();
    AAHexa::new(
        load_vec3(obj, "pos"),
        load_f64(obj, "x"),
        load_f64(obj, "y"),
        Rc::new(load_material(obj, "mat")),
    )
}

// AABox
fn load_aabox<'a>(root: &Json, key: &str) -> AABox<'a> {
    let obj = root.find(key).unwrap();
    AABox::new(
        load_vec3(obj, "pos"),
        load_vec3(obj, "dim"),
        Rc::new(load_material(obj, "mat")),
        load_bool(obj, "skybox"),
    )
}

// AARect
fn load_aarect(root: &Json, key: &str) -> AARect {
    let obj = root.find(key).unwrap();
    AARect::new(
        load_vec3(obj, "pos"),
        load_dir(obj, "dir"),
        load_vec3(obj, "dim"),
        Rc::new(load_material(obj, "mat")),
    )
}

// Dir
fn load_dir(root: &Json, key: &str) -> Dir {
    match root.find(key).unwrap().as_string().unwrap() {
        "left"   => Dir::Left,
        "right"  => Dir::Right,
        "top"    => Dir::Top,
        "bottom" => Dir::Bottom,
        "front"  => Dir::Front,
        "back"   => Dir::Back,
        _        => panic!("Not a Dir"),
    }
}

// Plane
fn load_plane(root: &Json, key: &str) -> Plane {
    let obj = root.find(key).unwrap();
    Plane::new(
        load_vec3(obj, "pos"),
        load_vec3(obj, "normal"),
        Rc::new(load_material(obj, "mat")),
    )
}

// Sphere
fn load_sphere(root: &Json, key: &str) -> Sphere {
    let obj = root.find(key).unwrap();
    Sphere::new(
        load_vec3(obj, "pos"),
        load_f64(obj, "radius"),
        Rc::new(load_material(obj, "mat")),
    )
}

// Material
fn load_material(root: &Json, key: &str) -> Material {
    let obj = root.find(key).unwrap();
    Material::new(
        load_color(obj, "color"),
        load_f64_or(obj, "spec", 0.),
        load_f64_or(obj, "diff", 1.),
        load_f64_or(obj, "refr", 0.),
        load_f64_or(obj, "refr-idx", 1.),
        load_f64_or(obj, "refl", 0.),
    )
}

// Color
fn load_color(root: &Json, key: &str) -> Color {
    let obj = root.find(key).unwrap();
    Color::new(
        load_f64(obj, "r"),
        load_f64(obj, "g"),
        load_f64(obj, "b"),
    )
}

fn load_color_or(root: &Json, key: &str, def: Color) -> Color {
    let obj = root.find(key);
    if obj.is_none() {
        return def;
    }
    Color::new(
        load_f64(obj.unwrap(), "r"),
        load_f64(obj.unwrap(), "g"),
        load_f64(obj.unwrap(), "b"),
    )
}

// Vec3
fn load_vec3(root: &Json, key: &str) -> Vec3 {
    let obj = root.find(key).unwrap();
    Vec3::new(
        load_f64(obj, "x"),
        load_f64(obj, "y"),
        load_f64(obj, "z"),
    )
}

// String
fn load_str<'a>(root: &'a Json, key: &str) -> &'a str {
    root.find(key).unwrap().as_string().unwrap()
}

// f64
fn load_f64(root: &Json, key: &str) -> f64 {
    root.find(key).unwrap().as_f64().unwrap()
}

fn load_f64_or(root: &Json, key: &str, def: f64) -> f64 {
    let obj = root.find(key);
    if obj.is_none() {
        return def;
    }
    obj.unwrap().as_f64().unwrap()
}

// u32
fn load_u32(root: &Json, key: &str) -> u32 {
    root.find(key).unwrap().as_i64().unwrap() as u32
}

fn load_u32_or(root: &Json, key: &str, def: u32) -> u32 {
    let obj = root.find(key);
    if obj.is_none() {
        return def;
    }
    obj.unwrap().as_i64().unwrap() as u32
}

// i32
fn load_i32(root: &Json, key: &str) -> i32 {
    root.find(key).unwrap().as_i64().unwrap() as i32
}

// bool
fn load_bool(root: &Json, key: &str) -> bool {
    root.find(key).unwrap().as_boolean().unwrap()
}
