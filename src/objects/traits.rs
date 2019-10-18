use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;

pub trait March {
    fn march(&self, point: Vec3) -> f64;
}

pub trait Trace {
    fn trace(&self, ray: Ray) -> (bool, f64, Vec3);
}
