use crate::structures::camera::Camera;
use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;
use crate::objects::traits::{March, Trace};

pub struct Scene {
    pub march: Vec<Box<March>>,
    pub trace: Vec<Box<Trace>>,
    pub camera: Camera,
}

impl Scene {
    pub fn new(camera: Camera) -> Scene {
        Scene {march: vec![], trace: vec![], camera: camera}
    }

    pub fn add_march(&mut self, march: Marchable) {
        self.march.push(march);
    }

    pub fn add_trace(&mut self, trace: Traceable) {
        self.trace.push(trace);
    }
}
