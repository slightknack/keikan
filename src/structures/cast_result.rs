use std::f64;

use crate::structures::vec3::Vec3;
use crate::structures::material::Material;

#[derive(Debug, Copy, Clone)]
pub struct CastResult {
    pub hit: bool,
    pub distance: f64,
    pub normal: Vec3,
    pub material: Material,
}

impl CastResult {
    pub fn new(hit: bool, distance: f64, normal: Vec3, material: Material) -> CastResult {
        CastResult {
            hit: hit,
            distance: distance,
            normal: normal,
            material: material,
        }
    }

    pub fn worst() -> CastResult {
        CastResult {
            hit: false,
            distance: f64::MAX,
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: Material::blank(),
        }
    }

    pub fn unpack(&self) -> (bool, f64, Vec3, Material) {
        (self.hit, self.distance, self.normal, self.material)
    }
}
