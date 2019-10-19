use crate::structures::vec3::Vec3;

// TODO: derive debug.. etc. for other structs
#[derive(Debug, Copy, Clone)]
pub struct Material {
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

    pub fn sky() -> Material {
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

    pub fn blank() -> Material {
        Material::sky()
        // Material {
        //     color: Vec3::new(0.0, 0.0, 0.0),
        //     emission: 0.0,
        //
        //     metallic: 0.0,
        //     specular: 0.0,
        //     roughness: 0.0,
        //
        //     transmission: 0.0,
        //     ior: 0.0,
        // }
    }
}
