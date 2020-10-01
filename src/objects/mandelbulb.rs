use std::f64;

use crate::structures::vec3::Vec3;
use crate::structures::material::Material;
use crate::objects::traits::March;

pub struct Mandelbulb {
    pub position: Vec3,
    pub power: f64,
    pub iterations: usize,
    pub material: Material,
}

fn length(x: f64, y: f64) -> f64 {
    (x * x + y * y).sqrt()
}

impl Mandelbulb {
    pub fn new(position: Vec3, power: f64, iterations: usize, material: Material) -> Mandelbulb {
        Mandelbulb {
            position: position,
            power: power,
            iterations: iterations,
            material: material,
        }
    }
}

impl March for Mandelbulb {
    fn material(&self) -> Material { self.material }

    fn march(&self, point: Vec3) -> f64 {
        let mut zn = point.clone() - self.position; // added self.position
        let mut rad = 0.0;
        let mut hit = 0.0;
        let mut d = 1.0;

        for _ in 0..self.iterations {
            rad = zn.length();

            if rad > 2.0 {
                hit = 0.5 * rad.ln() * rad / d;
            } else {
                let th = length(zn.x, zn.y).atan2(zn.z);
                let phi = zn.y.atan2(zn.x);
                let rado = rad.powi(8);
                d = rad.powi(7) * 7.0 * d + 1.0;

                let sint = (th * self.power).sin();
                zn.x = rado * sint * (phi * self.power).cos();
                zn.y = rado * sint * (phi * self.power).sin();
                zn.z = rado * (th * self.power).cos();
                zn = zn + (point - self.position); // added self.position
            }
        }

        return hit;
    }
}
