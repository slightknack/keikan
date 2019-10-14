use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;
use crate::structures::camera::Camera;
use crate::structures::scene::{Scene, Marcher, Tracer};
use crate::structures::material::Material;

fn march_sphere(position: Vec3, radius: f64) -> Marcher {
    fn distance(point: Vec3) -> f64 {
        (point - position).length() - radius
    }

    return distance;
}

fn trace_sphere(position: Vec3, radius: f64) -> Tracer {
    fn intersects(ray: Ray) -> (bool, f64, Vec3) {
        let oc = ray.origin - position;

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - radius * radius;

        let hit = (b * b) - (4.0 * a * c) > 0.0;

        // TODO: distance, normal
        return (hit, distance, normal);
    }

    return intersects;
}

pub fn make_scene() -> Scene {
    let camera = Camera::new(
        Vec3::new(0.0, 5.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let scene = Scene::new(camera);

    scene.add_march((
        march_sphere(Vec3::new(0.0, 0.0, 0.0), 1.0),
        Material {
            color: Vec3::new(1.0, 0.0, 0.0), // red
            emission: 0.0, // not a light!

            // shiny plastic surface
            metallic: 0.0,
            specular: 0.5,
            roughness: 0.0,

            // see-through
            transmission: 0.5,
            ior: 0.0,
        }
    ));

    scene.add_trace((
        trace_sphere(Vec3::new(-2.0, 0.0, 0.0), 1.0)
        Material {
            color: Vec3::new(1.0, 1.0, 1.0), // white
            emission: 2.0, // a light!

            // shiny plastic surface
            metallic: 0.0,
            specular: 0.0,
            roughness: 0.0,

            // not transparent
            transmission: 0.0,
            ior: 0.0,
        }
    ));

    scene.add_march(
        march_sphere(Vec3::new(2.0, 0.0, 0.0), 1.0),
        Material {
            color: Vec3::new(0.5, 0.5, 1.0), // blueish
            emission: 0.0, // not a light!

            // metallic
            metallic: 1.0,
            specular: 0.2,
            roughness: 0.0,

            // not transparent
            transmission: 0.0,
            ior: 0.0,
        }
    );

    return scene;
}
