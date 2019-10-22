use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub ray: Ray,
    pub up: Vec3,
}

impl Camera {
    pub fn new(from: Vec3, to: Vec3, up: Vec3) -> Camera{
        let f = (to - from).unit();

        println!("f {:?}", f);

        Camera {
            ray: Ray::new(from, f),
            up: up,
        }
    }
}
