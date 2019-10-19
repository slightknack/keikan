use crate::structures::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn through(origin: Vec3, to: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: (origin - to).unit()
        }
    }

    pub fn point_at(&self, distance: &f64) -> Vec3 {
        self.origin + self.direction * (*distance)
    }
}

impl PartialEq for Ray {
    fn eq(&self, other: &Ray) -> bool {
        (self.origin == other.origin) && (self.direction == other.direction)
    }
}
