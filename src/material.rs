use super::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
struct Material {
    pub color: Vec3, // color
    pub emission: f64, // how strong?

    pub metallic: f64,
    pub specular: f64,
    pub roughness: f64,

    pub IOR: f64,
    pub Transmission: f64,
}

impl Material {
    
}
