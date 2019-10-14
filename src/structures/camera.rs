use structures::vec3::Vec3;
use structures::ray::Ray;

struct Camera {
    camera: Ray,
    up: Vec3,
}

impl Camera {
    fn new(from: Vec3, to: Vec3, up: Vec3) -> Camera{
        w = (from - to).unit();
        u = (up.cross(w)).unit();
        v = w.cross(u);

        Camera {
            camera: Ray::new(v, ),
            up: up,
        }
    }
}
