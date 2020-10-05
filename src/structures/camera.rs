use std::io::Write;
use std::thread;
use std::sync::Arc;
use num_cpus;

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

    pub fn render(self, scene: Scene)  -> Vec<Vec<Vec3>> {
        // display rendering information
        println!("Render Information\n");

        println!("rendering {} pixel(s):", self.reso.0 * self.reso.1);
        println!(" - {} row(s)", self.reso.1);
        println!(" - {} column(s)\n", self.reso.0);

        println!("taking {} samples(s) per pixel:", self.aa * self.branch.pow(self.bounces as u32));
        println!(" - {} base sample(s) for AA", self.aa);
        println!(" - {} bounce(s) per sample", self.bounces);
        println!(" - {} branch(es) per bounce\n", self.branch);

        println!("scene has {} object(s):", scene.trace.len() + scene.march.len());
        println!(" - {} traced object(s)", scene.trace.len());
        println!(" - {} marched object(s)\n", scene.march.len());

        let camera = Arc::new(self);
        let scene  = Arc::new(scene);

        let num_workers = num_cpus::get();
        println!("automatically detected {} cpu core(s):", num_workers);

        let mut workers = vec![];
        for worker in 0..num_workers {
            let start = self.height() * worker / num_workers;
            let stop   = self.height() * (worker + 1) / num_workers;

            let camera_clone = Arc::clone(&camera);
            let scene_clone  = Arc::clone(&scene);

            workers.push(thread::spawn(move || {
                Camera::section(camera_clone, scene_clone, start, stop, worker + 1)
            }));
        }

        let mut result = vec![];

        for worker in workers {
            result.append(&mut worker.join().expect("thread failed to return value"))
        }

        println!();
        return result;
    }

    pub fn section(
        self: Arc<Self>, scene: Arc<Scene>,
        start: usize, stop: usize, id: usize,
    ) -> Vec<Vec<Vec3>> {
        let mut rng = rand::thread_rng();
        let mut image = vec![];

        for y in start..stop {
            let mut row = vec![];

            for x in 0..self.width() {
                row.push(
                    sample(&scene, &self, &mut rng, x as f64, (self.height() - y) as f64)
                );
            }

            image.push(row);
            print!("\r - worker {} is {}% done", id, (y - start + 1) * 100 / (stop - start));
            std::io::stdout().flush().ok().expect("Could not flush stdout");
        }

        println!();
        return image;
    }
}
