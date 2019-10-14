fn march_sphere(position: Vec3, radius: f64) {
    fn distance(point: Vec3) {
        (point - position).length() - radius
    }

    return distance;
}

fn trace_sphere(position: Vec3, radius: f64) {
    fn intersects(ray: Ray) {
        let oc = ray.origin - position;

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - radius * radius;

        let discriminant = (b * b) - (4 * a * c);

        discriminant > 0
    }
}

fn make_scene() -> Scene {
    let camera = Camera::new()
    let scene = Scene::new();

    scene.add_march(
        march_sphere(Vec3::new(1.0, 0.0, 0.0), 1.0)
    );

    scene.add_trace(
        trace_sphere(Vec3::new(-1.0, 0.0, 0.0), 1.0)
    );
}
