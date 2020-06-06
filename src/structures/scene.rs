use std::sync::Arc;
use std::io::Write;

use crate::structures::camera::Camera;
use crate::structures::vec3::Vec3;
use crate::objects::traits::{ March, Trace };
use crate::render::render;

pub struct Scene {
    pub march: Vec<Arc<dyn March>>,
    pub trace: Vec<Arc<dyn Trace>>,
    pub camera: Camera,
}

impl Scene {
    pub fn new(camera: Camera) -> Scene {
        Scene { march: vec![], trace: vec![], camera: camera }
    }

    pub fn add_march(&mut self, march: impl March + 'static) {
        self.march.push(Arc::new(march));
    }

    pub fn add_trace(&mut self, trace: impl Trace + 'static) {
        self.trace.push(Arc::new(trace));
    }

    pub fn render(&self) -> Vec<Vec<Vec3>> {
        let mut image = vec![];

        for y in 0..self.camera.height() {
            let mut row = vec![];

            print!("\rrow {} / {} ", y + 1, self.camera.height());
            std::io::stdout().flush().ok().expect("Could not flush stdout");

            for x in 0..self.camera.width() {
                row.push(render(
                    &self,
                    (x as f64, (self.camera.height() - y) as f64),
                ));
            }

            image.push(row);
        }

        println!();
        return image;
    }
}
