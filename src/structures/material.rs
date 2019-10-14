use super::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
struct Material {
    pub color: Vec3, // color
    pub emission: f64, // how strong?

    pub metallic: f64,
    pub specular: f64,
    pub roughness: f64,

    pub transmission: f64,
    pub ior: f64,
}

// ior and specular are correlated, remove one or the other?

impl Material {
    // TODO

    fn sky() -> Material {
        Material {
            color: Vec3::new(0.6, 0.8, 1.0),
            emission: 1.0,

            metallic: 0.0,
            specular: 0.0,
            roughness: 0.0,

            transmission: 0.0,
            ior: 0.0,
        }
    }
}
