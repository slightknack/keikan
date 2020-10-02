use std::rc::Rc;

use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;
use crate::structures::material::Material;
use crate::structures::cast::Cast;
use crate::render::EPSILON;

pub trait Trace {
    fn material(&self) -> Material;
    fn trace(&self, ray: Ray) -> Option<(f64, Vec3)>; // distance, normal
}

impl dyn Trace {
    pub fn hit(trace: &Vec<Rc<dyn Trace>>, ray: Ray) -> Option<Cast> {
        let mut best: Option<Cast> = None;

        for object in trace.iter() {
            let (distance, normal) = match object.trace(ray) {
                Some(v) => v,
                None => continue,
            };

            let visible = distance > EPSILON;
            let closer = if let Some(cast) = best { distance < cast.distance  } else { true };
            let cast = Cast { distance, normal, material: object.material() };

            if visible && closer {
                best = Some(cast);
            }
        }

        return best;
    }
}
