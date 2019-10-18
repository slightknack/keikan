use crate::structures::material::Material;
use crate::structures::camera::Camera;
use crate::structures::scene::Scene;
use crate::structures::vec3::Vec3;
use crate::objects::sphere::Sphere;

const PLASTIC: Material = Material {
    color: Vec3::new(1.0, 0.0, 0.0), // red
    emission: 0.0, // not a light!

    // shiny plastic surface
    metallic: 0.0,
    specular: 0.5,
    roughness: 0.0,

    // see-through
    transmission: 0.5,
    ior: 0.0,
};

const LIGHT: Material = Material {
    color: Vec3::new(1.0, 1.0, 1.0), // white
    emission: 2.0, // a light!

    // shiny plastic surface
    metallic: 0.0,
    specular: 0.0,
    roughness: 0.0,

    // not transparent
    transmission: 0.0,
    ior: 0.0,
};

const METAL: Material = Material {
    color: Vec3::new(0.5, 0.5, 1.0), // blueish
    emission: 0.0, // not a light!

    // metallic
    metallic: 1.0,
    specular: 0.2,
    roughness: 0.0,

    // not transparent
    transmission: 0.0,
    ior: 0.0,
};

pub fn make_scene() -> Scene {
    let camera = Camera::new(
        Vec3::new(0.0, 5.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let scene = Scene::new(camera);

    scene.add_march(Sphere::new(Vec3::new( 0.0, 0.0, 0.0), 1.0, PLASTIC));
    scene.add_trace(Sphere::new(Vec3::new(-2.0, 0.0, 0.0), 1.0, LIGHT));
    scene.add_march(Sphere::new(Vec3::new( 2.0, 0.0, 0.0), 1.0, METAL));

    return scene;
}
