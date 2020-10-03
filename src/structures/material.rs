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
    // pub ior: f64,
}

// ior and specular are correlated, remove one or the other?

impl Material {
    // TODO

    pub fn sky() -> Material {
        Material::emissive(Vec3::new(0.2, 1.0, 0.8), 0.6)
    }

    pub fn emissive(color: Vec3, emission: f64) -> Material {
        Material {
            color,
            emission,

            metallic: 0.0,
            specular: 0.0,
            roughness: 0.0,

            transmission: 0.0,
            // ior: 1.0,
        }
    }

    pub fn metal(color: Vec3, specular: f64, roughness: f64) -> Material {
        Material {
            color,
            emission: 0.0,

            metallic: 1.0,
            specular,
            roughness,

            transmission: 0.0,
        }
    }

    pub fn dielectric(color: Vec3, specular: f64, roughness: f64) -> Material {
        Material {
            color,
            emission: 0.0,

            metallic: 0.0,
            specular,
            roughness,

            transmission: 0.0,
        }
    }

    // TODO: transparent
    // convert ior to specular using polynomial approx
}
