use rand::random;

use f64;
use f64::consts::PI;

use structures::vec3::Vec3;
use structures::ray::Ray;
use structures::camera::Camera;
use structures::scene::{Scene, Marchable, Traceable};
use structures::material::Material;

fn make_ray(fov: f64, ratio: f64, uv: [f64, 2]) -> Ray {
    // I apologize for this garbage
    let xy = [uv[0] - ratio * 0.5, uv[1] - 0.5];
    let z = 1.0 / (FOV.to_radians() / 2.0).tan();
    return (Vec3::new(xy[0], xy[1], -z)).unit();
}

fn hit_trace(trace: Vec<Traceable>, ray: Ray) -> (bool, f64, Vec3, Material) {

    // find closest object
    // get that object's material and normal
    // return

        // write this code

    for

}

fn hit_march(march: Vec<Marchable>, ray: Ray) -> (bool, f64, Vec3, Material) {

    // march the field
    // if hit, iterate through functions and find closest
    // get that function's material,
    // sample it's normal,
    // return

    fn sdf(point: Vec3) {
        for marchable in march {

        }
    }

}

fn cast_ray(scene: Scene, ray: Ray) -> (bool, f64, Vec3, Material) {
    let (march_hit, march_dist, march_norm, march_mat) = hit_march(scene.march);
    let (trace_hit, trace_dist, trace_norm, trace_mat) = hit_trace(scene.trace);

    // nothing was hit, so return the sky
    if !march_hit && !trace_hit {
        return false, f64::MAX, ray.direction.unit(), Material::sky()
    }

    let hit = true;

    let (dist, norm, mat) = match trace_hit && !march_hit || trace_dist <= march_dist {
        true => (
            trace_dist,
            trace_norm,
            trace_mat,
        ),
        false => (
            march_dist,
            march_norm,
            march_mat,
        ),
    }

    return (hit, dist, norm, mat);
}

fn sample_sphere() -> Vec3 {
    // speed up with `let mut rng = rand::thread_rng()` in closure?

    let mut rng = rand::thread_rng()
    let mut point: Vec3;

    while (point.length_squared() >= 1.0) {
        point = Vec3::new(
            rng.gen() * 2.0 - 1.0,
            rng.gen() * 2.0 - 1.0,
            rng.gen() * 2.0 - 1.0,
         );
    }

    return point;
}

fn color(scene: Scene, ray: Ray, samples: u32, max_bounce: u32) -> Vec3 {
    let hit, distance, normal, material = cast_ray(scene, ray);

    // return the sky
    if !hit || max_bounce = 0 {
        return material.color * material.emission;
    }

    // time for some recursion...
    let position = ray.point_along(distance);

    // we need to get 3 things:
    // a reflection, a diffuse, and a transmission

    let mut diffuse = Vec3::new(0.0, 0.0, 0.0);

    let mut color: Vec3;
    let mut bounce: Ray;

    // diffuse:
    for _ in 0..samples {
        bounce = diffuse + Ray::through(position, position + sample_sphere()),
        sample = color(scene, bounce, samples, max_bounce - 1);

        diffuse = diffuse + (bounce.dot(normal) * sample);
    }

    diffuse = diffuse / samples;

    // reflection


    // use the material to calculate:
    // - color
}

fn render(scene: Scene, uv: [f64; 2], resolution: [usize; 2]) -> Vec3 {
    // make ray
    let ray = make_ray(120.0, resolution[0] / resolution[1], uv);

    // cast ray
    color(scene, ray, samples, max_bounce);
}
