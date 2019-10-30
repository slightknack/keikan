use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;
use crate::structures::material::Material;
use crate::objects::traits::{ March, Trace };

#[derive(Debug, Copy, Clone)]
pub struct Plane {
    pub position: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Plane {
    pub fn new(position: Vec3, normal: Vec3, material: Material) -> Plane {
        Plane {
	    position: position,
            normal: normal,
            material: material,
        }
    }
}

impl Trace for Plane {
    fn material(&self) -> Material { self.material }

    fn trace(&self, ray: Ray) -> (bool, f64, Vec3) {
        let denom = self.normal.dot(ray.direction);
        if (denom.abs > 0.0) {
            let t = (self.position - ray.origin).dot(self.normal) / denom;
	    
            if (t >= 0.0) {
                 return (true, t, self.normal);
            }
        }

        return (false, f64::MAX, self.normal);
    }
}

impl March for Sphere {
    fn material(&self) -> Material { self.material }

    fn march(&self, point: Vec3) -> f64 {
        (point - self.position).length() - self.radius // TODO modulo with 6 for infinite rep.
    }
}
