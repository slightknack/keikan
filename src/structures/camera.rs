use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub ray: Ray, // position and direction of camera
    pub up: Vec3, // up vector of camera
    pub fov: f64, // frame of view, in degrees
    pub reso: (usize, usize), // resolution of camera, in pixels
}

impl Camera {
    pub fn new(from: Vec3, to: Vec3, up: Vec3, fov: f64, reso: (usize, usize)) -> Camera {
        let f = (to - from).unit();
        Camera { ray: Ray::new(from, f), up, fov, reso }
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
}
