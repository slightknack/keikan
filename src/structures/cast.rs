use std::f64;

use crate::structures::vec3::Vec3;
use crate::structures::material::Material;

#[derive(Debug, Copy, Clone)]
pub struct Cast {
    pub distance: f64,
    pub normal: Vec3,
    pub material: Material,
}
