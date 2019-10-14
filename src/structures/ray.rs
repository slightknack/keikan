use structures::vec3::Vec3;
use structures::ray::Ray;

struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    fn through(origin: Vec3, to: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: (origin - to).unit()
        }
    }

    fn point_along(&self, &distance: float) -> Vec3 {
        self.origin + self.direction * distance;
    }
}

impl PartialEq for Ray {
    fn eq(&self, other: &Ray) -> bool {
        (self.origin == other.origin) && (self.direction == other.direction)
    }
}
