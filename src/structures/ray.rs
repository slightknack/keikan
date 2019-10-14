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
}

impl PartialEq for Ray {
    fn eq(&self, other: &Ray) -> bool {
        (self.origin == other.origin) && (self.direction == other.direction)
    }
}
