use std::rc::Rc;

use crate::structures::material::Material;
use crate::objects::march::March;
use crate::objects::trace::Trace;

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
}
