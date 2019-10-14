use crate::structures::camera::Camera;
use crate::structures::material::Material;
use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;

pub type Marcher = fn(Vec3) -> f64;
pub type Tracer = fn(Ray) -> (bool, f64, Vec3);

pub type Marchable = (Marcher, Material);
pub type Traceable = (Tracer, Material);

pub struct Scene {
    pub march: Vec<Marchable>,
    pub trace: Vec<Traceable>,
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
