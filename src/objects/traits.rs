use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;
use crate::structures::material::Material;

// I see duplicate code... hmm...

pub trait March {
    fn material(&self) -> Material;
    fn march(&self, point: Vec3) -> f64;
}

pub trait Trace {
    fn material(&self) -> Material;
    fn trace(&self, ray: Ray) -> (bool, f64, Vec3);
}
