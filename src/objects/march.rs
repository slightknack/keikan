use crate::structures::material::Material;
use crate::structures::cast::Cast;
use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;

use crate::render::EPSILON;
pub const MAX_STEPS: usize = 64;
pub const MAX_DEPTH: f64 = 40.0;

pub trait March: Send + Sync {
    fn material(&self) -> Material;
    fn march(&self, point: Vec3) -> f64; // distance to nearest point
}

impl dyn March {
    fn sdf(point: Vec3, march: &Vec<Box<dyn March>>) -> (f64, Material) {
        let mut min = f64::MAX;
        let mut mat = Material::sky();

        for object in march.iter() {
            let distance = object.march(point);

            if distance <= min {
                min = distance;
                mat = object.material();
            }
        }

        return (min, mat);
    }

    // TODO: replace with faster normal epsilon sample technique
    fn normal(p: Vec3, march: &Vec<Box<dyn March>>) -> Vec3 {
        Vec3::new(
            March::sdf(Vec3::new(p.x + EPSILON, p.y, p.z), march).0 - March::sdf(Vec3::new(p.x - EPSILON, p.y, p.z), march).0,
            March::sdf(Vec3::new(p.x, p.y + EPSILON, p.z), march).0 - March::sdf(Vec3::new(p.x, p.y - EPSILON, p.z), march).0,
            March::sdf(Vec3::new(p.x, p.y, p.z + EPSILON), march).0 - March::sdf(Vec3::new(p.x, p.y, p.z - EPSILON), march).0,
        ).unit()
    }

    pub fn hit(march: &Vec<Box<dyn March>>, ray: Ray) -> Option<Cast> {
        let mut depth = EPSILON;

        for _step in 0..MAX_STEPS {
            let point = ray.point_at(&depth);
            let (distance, material) = March::sdf(point, march);

            if distance <= EPSILON {
                let normal = March::normal(point, march);
                return Some(Cast { distance: depth, normal, material });
            }

            if distance >= MAX_DEPTH { break; }
            depth += distance;
        }
        return None;
    }
}
