use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub ray: Ray,
    pub up: Vec3,
    pub fov: f64,
    pub reso: (usize, usize),
}

impl Camera {
    pub fn new(from: Vec3, to: Vec3, up: Vec3, reso: (usize, usize)) -> Camera {
        let f = (to - from).unit();

        Camera {
            ray: Ray::new(from, f),
            up: up,
            fov: 10.0,
            reso: reso,
        }
    }

    fn ratio(&self) -> f64 {
        (self.width() as f64) / (self.height() as f64)
    }

    pub fn width(&self)  -> usize { self.reso.0 }
    pub fn height(&self) -> usize { self.reso.1 }

    // uv is resolution coordinate + noise
    pub fn make_ray(&self, xy: (f64, f64)) -> Ray {
        // normalize coordinates
        let mut uv = (xy.0 / (self.width() as f64), xy.1 / (self.height() as f64));
        uv.0 *= self.ratio();
        uv = (uv.0 - self.ratio() * 0.5, uv.1 - 0.5);

        // find direction
        let z = 1.0 / (self.fov.to_radians() / 2.0).tan();
        let dir = Vec3::new(uv.0, uv.1, -z).unit();

        // translate the ray
        let f = self.ray.direction;
        let s = (f.cross(&self.up)).unit();
        let u = s.cross(&f);
        let r = dir;

        // return the new ray.
        return Ray::new(
            self.ray.origin,
            Vec3::new( // cross product
                (r.x * s.x) + (r.y * u.x) + (r.z * -f.x),
                (r.x * s.y) + (r.y * u.y) + (r.z * -f.y),
                (r.x * s.z) + (r.y * u.z) + (r.z * -f.z),
            ),
        )
    }
}
