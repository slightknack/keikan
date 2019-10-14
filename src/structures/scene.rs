type Marchable = (fn(Vec3) -> f64, Material);
type Traceable = (fn(Ray) -> (bool, f64, Vec3), Material);

struct Scene {
    pub march: Vec<Marchable>,
    pub trace: Vec<Traceable>,
    pub camera: Camera,
}

impl Scene {
    fn new(camera: Camera) -> Scene {
        Scene {march: vec![], trace: vec![], camera: camera}
    }

    fn add_march(&mut self, &march: Marchable) {
        self.march.push(march);
    }

    fn add_trace(&mut self, &march: Traceable) {
        self.trace.push(trace);
    }
}
