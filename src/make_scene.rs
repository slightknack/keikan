use crate::structures::material::Material;
use crate::structures::camera::Camera;
use crate::structures::scene::Scene;
use crate::structures::vec3::Vec3;
use crate::objects::sphere::Sphere;
use crate::objects::mandelbulb::Mandelbulb;

pub fn make_scene() -> Scene {
    let camera = Camera::new(
        Vec3::new(-2.0, 1.0, 4.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut scene = Scene::new(camera);

    let plastic = Material {
        color: Vec3::new(0.9, 0.9, 0.9), // red
        emission: 0.0, // not a light!

        // plastic surface
        metallic: 0.0,
        specular: 0.2,
        roughness: 0.0,

        // see-through
        transmission: 0.0,
        ior: 0.0,
    };

    let light = Material {
        color: Vec3::new(1.0, 1.0, 0.6), // white
        emission: 10.0, // a light!

        // shiny plastic surface
        metallic: 0.0,
        specular: 0.3,
        roughness: 0.0,

        // not transparent
        transmission: 0.0,
        ior: 0.0,
    };

    let metal = Material {
        color: Vec3::new(0.9, 0.9, 0.7), // gold
        emission: 0.0, // not a light!

        // metallic
        metallic: 1.0,
        specular: 0.0,
        roughness: 0.0,

        // not transparent
        transmission: 0.0,
        ior: 0.0,
    };

    scene.add_trace(Sphere::new(Vec3::new(4.0, 4.0, 4.0), 2.0, light));
    scene.add_march(Mandelbulb::new(Vec3::new(0.0, 0.0, 0.0), 8.0, 10, metal));
    scene.add_trace(Sphere::new(Vec3::new(0.0, -101.0, 0.0), 100.0, plastic));

    return scene;
}
