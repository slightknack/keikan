use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub ray: Ray,
    pub up: Vec3,
}

impl Camera {
    pub fn new(from: Vec3, to: Vec3, up: Vec3) -> Camera{
        let w = (from - to).unit();
        let u = (up.cross(&w)).unit();
        let v = w.cross(&u);

        Camera {
            ray: Ray::new(from, v),
            up: up,
        }
    }
}
