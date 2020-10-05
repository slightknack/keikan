use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;
use crate::structures::material::Material;
use crate::objects::trace::Trace;

use crate::render::EPSILON;

// TODO: maybe make material a wrapper around marches and traces?
pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    material: Material,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, material: Material) -> Triangle {
        Triangle { a, b, c, material }
    }
}

impl Trace for Triangle {
    fn material(&self) -> Material {
        self.material
    }

    fn trace(&self, ray: Ray) -> Option<(f64, Vec3)> {
        // triangle degined by vertices self.a, self.b and self.c
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        let rov0 = ray.origin - self.a;
        let n = ab.cross(&ac);
        let q = rov0.cross(&ray.direction);
        let d = 1.0 / (ray.direction.dot(&n));
        let u = d * (0.0 - q).dot(&ac);
        let v = d * q.dot(&ab);
        let t = d * (0.0 - n).dot(&rov0);
        let hit = u.min(v.min(t.min(1.0 - (u + v)))) > EPSILON;

        return if hit {
            Some(((Vec3::new(t, u, v) - ray.origin).length(), n.unit()))
        } else {
            None
        }
    }
}
