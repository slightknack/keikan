use std::sync::Arc;
use std::rc::Rc;
use std::io::Write;

use crate::structures::camera::Camera;
use crate::structures::vec3::Vec3;
use crate::structures::material::Material;
use crate::objects::march::March;
use crate::objects::trace::Trace;
use crate::render::sample;

pub struct Scene {
    pub march: Vec<Rc<dyn March>>,
    pub trace: Vec<Rc<dyn Trace>>,
    pub bg: Material,
}

impl Scene {
    pub fn empty() -> Scene {
        Scene { march: vec![], trace: vec![], bg: Material::sky() }
    }

    pub fn add_march(&mut self, march: Box<dyn March>) {
        self.march.push(Rc::from(march));
    }

    pub fn add_trace(&mut self, trace: Box<dyn Trace>) {
        self.trace.push(Rc::from(trace));
    }

    pub fn render(&self, camera: Camera) -> Vec<Vec<Vec3>> {
        let mut image = vec![];

        for y in 0..camera.height() {
            let mut row = vec![];

            print!("\rrow {}/{}, {}%", y + 1, camera.height(), y * 100 / camera.height());
            std::io::stdout().flush().ok().expect("Could not flush stdout");

            for x in 0..camera.width() {
                row.push(sample(
                    &self,
                    &camera,
                    x as f64,
                    (camera.height() - y) as f64,
                ));
            }

            image.push(row);
        }

        println!();
        return image;
    }
}
