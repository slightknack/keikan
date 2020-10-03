use std::io::Write;

use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;
use crate::structures::scene::Scene;
use crate::render::sample;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub ray:  Ray, // position and direction of camera
    pub up:   Vec3, // up vector of camera
    pub fov:  f64, // frame of view, in degrees
    pub reso: (usize, usize), // resolution of camera, in pixels

    pub aa:      usize, // samples per pixel
    pub branch:  usize, // branches per bounce (tree-based path tracing)
    pub bounces: usize, // (maximum) number of bounces
}

impl Camera {
    pub fn new(
        from: Vec3, to: Vec3, up: Vec3,
        fov: f64, reso: (usize, usize),
        aa: usize, branch: usize, bounces: usize,
    ) -> Camera {
        let f = (to - from).unit();
        Camera {
            ray: Ray::new(from, f),
            up, fov, reso,
            aa, branch, bounces,
        }
    }

    fn ratio(&self) -> f64 {
        (self.width() as f64) / (self.height() as f64)
    }

    pub fn width(&self)  -> usize { self.reso.0 }
    pub fn height(&self) -> usize { self.reso.1 }

    // uv is resolution coordinate + noise
    // x, y is pixel location on camera sensor
    pub fn make_ray(&self, x: f64, y: f64) -> Ray {
        // normalize coordinates
        let (mut u, v) = (x / (self.width() as f64), y / (self.height() as f64));
        u *= self.ratio();
        let (u, v) = (u - self.ratio() * 0.5, v - 0.5);

        // find direction
        let z = 1.0 / (self.fov.to_radians() / 2.0).tan();
        let dir = Vec3::new(u, v, -z).unit();

        // translate the ray
        let f = self.ray.direction;
        let s = (f.cross(&self.up)).unit();
        let u = s.cross(&f);
        let r = dir;

        // return the new ray.
        return Ray::new(
            self.ray.origin,
            // cross product
            Vec3::new(
                (r.x * s.x) + (r.y * u.x) + (r.z * -f.x),
                (r.x * s.y) + (r.y * u.y) + (r.z * -f.y),
                (r.x * s.z) + (r.y * u.z) + (r.z * -f.z),
            ),
        )
    }

    pub fn render(&self, scene: Scene) -> Vec<Vec<Vec3>> {
        let mut rng = rand::thread_rng();
        let mut image = vec![];

        for y in 0..self.height() {
            let mut row = vec![];

            print!("\rrow {}/{}, {}%", y + 1, self.height(), (y + 1) * 100 / self.height());
            std::io::stdout().flush().ok().expect("Could not flush stdout");

            for x in 0..self.width() {
                row.push(sample(
                    &scene,
                    &self,
                    &mut rng,
                    x as f64,
                    (self.height() - y) as f64,
                ));
            }

            image.push(row);
        }

        println!();
        return image;
    }
}
