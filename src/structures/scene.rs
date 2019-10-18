use std::any::Any;

use crate::structures::camera::Camera;
use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;
use crate::objects::traits::{March, Trace};

// todo: figure out how to make scene for every object.

pub struct Scene {
    pub march: Vec<'static &dyn Any>,
    pub trace: Vec<'static &dyn Any>,
    pub camera: Camera,
}

impl Scene {
    pub fn new(camera: Camera) -> Scene {
        Scene {march: vec![], trace: vec![], camera: camera}
    }

    pub fn add_march(&mut self, march: &dyn Any) {
        self.march.push(march);
    }

    pub fn add_trace(&mut self, trace: &dyn Any) {
        self.trace.push(trace);
    }
}
