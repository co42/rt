use num::complex::{ Complex, Complex64 };

#[allow(dead_code)]
pub struct Point {
    pub iter: u32,
    pub z:    Complex64,
}

impl Point {
    #[allow(dead_code)]
    fn new(iter: u32, z: Complex64) -> Point {
        Point { iter: iter, z: z }
    }
}

#[allow(dead_code)]
pub fn mandelbrot(x: f64, y: f64, maxiter: u32) -> Point {
    let mut z = Complex::new(0., 0.);
    let c = Complex::new(x, y);
    let mut iter = 0;
    while iter < maxiter && z.norm_sqr() < 4. {
        z = z * z + c;
        iter += 1;
    }
    Point::new(iter, z)
}
