use std::sync::Arc;
use std::io::Write;

use crate::structures::camera::Camera;
use crate::structures::vec3::Vec3;
use crate::objects::traits::{ March, Trace };
use crate::render::render;

pub struct Scene {
    pub march: Vec<Arc<dyn March>>,
    pub trace: Vec<Arc<dyn Trace>>,
}

impl Scene {
    pub fn empty() -> Scene {
        Scene { march: vec![], trace: vec![] }
    }

    pub fn add_march(&mut self, march: &Arc<dyn March>) {
        self.march.push(Arc::clone(march));
    }

    pub fn add_trace(&mut self, trace: &Arc<dyn Trace>) {
        self.trace.push(Arc::clone(trace));
    }

    pub fn render(&self, camera: Camera) -> Vec<Vec<Vec3>> {
        let mut image = vec![];

        for y in 0..camera.height() {
            let mut row = vec![];

            print!("\rrow {} / {} ", y + 1, camera.height());
            std::io::stdout().flush().ok().expect("Could not flush stdout");

            for x in 0..camera.width() {
                row.push(render(
                    &self,
                    (x as f64, (camera.height() - y) as f64),
                ));
            }

            image.push(row);
        }

        println!();
        return image;
    }
}
