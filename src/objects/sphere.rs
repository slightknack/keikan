use structures::vec3::Vec3;
use structures::ray::Ray;
use objects::traits::{March, Trace};

struct Sphere {
    pub position: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(position: Vec3, radius: f64, material: Material) {
        Sphere {
            position: position,
            radius: radius,
            material: material,
        }
    }
}

impl Trace for Sphere {
    fn trace(&self, ray: Ray) -> (bool, f64, Vec3) {
        let oc = ray.origin - self.position;

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        let hit = (b * b) - (4.0 * a * c) > 0.0;

        // TODO: distance, normal
        return (hit, distance, normal);
    }
}

impl March for Sphere {
    fn march(&self, point: Vec3) -> f64 {
        (point - self.position).length() - self.radius
    }
}
