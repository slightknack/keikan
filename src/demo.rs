use crate::structures::material::Material;
use crate::structures::camera::Camera;
use crate::structures::scene::Scene;
use crate::structures::vec3::Vec3;
use crate::objects::sphere::Sphere;
use crate::objects::plane::Plane;
use crate::objects::mandelbulb::Mandelbulb;
use crate::objects::triangle::Triangle;

// const RESOLUTION: (usize, usize) = (1440, 900);
const RESOLUTION: (usize, usize) = (720, 450);

pub fn scene() -> (Scene, Camera) {
    let camera = Camera::new(
        Vec3::new(-2.0, 0.6, 4.0),
        Vec3::new(0.0,  0.0, 1.0),
        Vec3::new(0.0,  1.0, 0.0),
        20.0,
        RESOLUTION,
        8, 1, 3,
    );

    let mut scene = Scene::empty();

    let plastic = Material::dielectric(Vec3::new(1.0, 1.0, 0.4), 0.0, 1.0);
    let light   = Material::emissive(Vec3::new(1.0, 1.0, 1.0), 3.0);
    let metal   = Material::metal(Vec3::new(0.9, 0.8, 0.5), 0.0, 0.0);

    let sphere = Sphere::new(
        Vec3::new(4.0, 4.0, 4.0),
        4.0,
        light
    );

    let bulb = Mandelbulb::new(Vec3::new(0.0, 0.0, 0.0), 8.0, 10, plastic);

    let plane = Plane::new(
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0,  1.0, 0.0),
        plastic
    );

    scene.add_trace(Box::new(sphere));
    scene.add_march(Box::new(bulb));
    // scene.add_trace(Box::new(plane));

    return (scene, camera);
}

pub fn shadow() -> (Scene, Camera) {
    let camera = Camera::new(
        Vec3::new(5.0, 2.2, 5.0),
        Vec3::new(0.0, 1.2, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        RESOLUTION,
        64, 1, 4,
    );

    let mut scene = Scene::empty();
    scene.bg = Material::emissive(Vec3::new(0.5, 0.5, 1.0), 0.5);

    let light  = Material::emissive(Vec3::new(1.0, 1.0, 1.0), 2.0);
    let chalk  = Material::dielectric(Vec3::new(0.5, 0.7, 0.5), 0.7, 0.05);
    let mirror = Material::metal(Vec3::new(0.9, 0.5, 0.5), 0.0, 0.01);

    let sphere = Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mirror);
    let lamp   = Sphere::new(Vec3::new(0.0, 2.0, 2.0), 0.5, light);
    let ground = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), chalk);

    scene.add_trace(Box::new(sphere));
    scene.add_trace(Box::new(lamp));
    scene.add_trace(Box::new(ground));

    return (scene, camera);
}

pub fn triangle() -> (Scene, Camera) {
    let camera = Camera::new(
        Vec3::new(5.0, 5.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        RESOLUTION,
        16, 4, 2,
    );

    let mut scene = Scene::empty();
    scene.bg = Material::emissive(Vec3::new(0.0, 0.0, 0.0), 0.0);

    let light  = Material::emissive(Vec3::new(1.0, 1.0, 1.0), 2.0);
    let chalk  = Material::dielectric(Vec3::new(0.5, 0.5, 0.5), 0.5, 1.0);

    let triangle = Triangle::new(
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 2.0, 0.0),
        chalk,
    );

    let lamp = Sphere::new(Vec3::new(2.0, 2.0, 2.0), 1.0, light);

    scene.add_trace(Box::new(lamp));
    scene.add_trace(Box::new(triangle));

    return (scene, camera);
}
