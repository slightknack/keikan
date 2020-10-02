use crate::structures::material::Material;
use crate::structures::camera::Camera;
use crate::structures::scene::Scene;
use crate::structures::vec3::Vec3;
use crate::objects::sphere::Sphere;
use crate::objects::plane::Plane;
use crate::objects::mandelbulb::Mandelbulb;

const RESOLUTION: (usize, usize) = (960, 580);

pub fn scene() -> (Scene, Camera) {
    let camera = Camera::new(
        Vec3::new(-2.0, 0.6, 4.0),
        Vec3::new(0.0,  0.0, 1.0),
        Vec3::new(0.0,  1.0, 0.0),
        20.0,
        RESOLUTION,
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
        Vec3::new(0.0, 6.2, 0.1),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        RESOLUTION,
    );

    let mut scene = Scene::empty();
    scene.bg = Material::emissive(Vec3::new(0.5, 0.5, 1.0), 0.2);

    let light = Material::emissive(Vec3::new(1.0, 1.0, 1.0), 2.0);
    let chalk = Material::dielectric(Vec3::new(0.5, 0.5, 0.5), 0.0, 0.8);
    let mirror = Material::metal(Vec3::new(0.7, 0.7, 0.7), 0.0, 0.2);

    // let sphere = Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mirror);
    let bulb = Mandelbulb::new(Vec3::new(0.0, 1.0, 0.0), 8.0, 10, mirror);
    let lamp   = Sphere::new(Vec3::new(0.0, 1.0, 2.0), 0.5, light);
    let ground = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), chalk);

    scene.add_march(Box::new(bulb));
    scene.add_trace(Box::new(lamp));
    scene.add_trace(Box::new(ground));

    return (scene, camera);
}
