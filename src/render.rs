use rand::Rng;

use std::f64;

use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;
use crate::structures::camera::Camera;
use crate::structures::material::Material;
use crate::structures::cast_result::CastResult;
use crate::objects::traits::{March, Trace};

// constants
const MAX_STEPS: u32 = 128;
const MAX_DEPTH: u32 = 512;
const MAX_BOUNCES: u32 = 4;
const SAMPLES: u32 = 16;
const EPSILON: f64 = 1.0 / 512.0;

fn hit_trace(trace: Vec<Box<March>>, ray: Ray) -> CastResult {
    let best: CastResult::worst();

    for object in trace {
        let (hit, distance, normal) = object.trace(ray);

        if hit && (best[0] == false || distance <= best[1]) {
            best = CastResult::new(hit, distance, normal, material);
        }
    }

    return best;
}

fn hit_march(march: Vec<Box<Trace>>, ray: Ray) -> CastResult {
    fn sdf(point: Vec3) -> (f64, Material) {
        let mut min = f64::MAX;
        let mut mat = Material::blank();

        for object in march {
            let distance = object.march(point);

            if distance <= min {
                min = distance;
                mat = material;
            }
        };

        return (min, mat);
    }

    fn normal(p: Vec3) -> Vec3 {
        return Vec3::new(
            sdf(Vec3::new(p.x + EPSILON, p.y, p.z)).0 - sdf(Vec3::new(p.x - EPSILON, p.y, p.z)).0,
            sdf(Vec3::new(p.x, p.y + EPSILON, p.z)).0 - sdf(Vec3::new(p.x, p.y - EPSILON, p.z)).0,
            sdf(Vec3::new(p.x, p.y, p.z + EPSILON)).0 - sdf(Vec3::new(p.x, p.y, p.z - EPSILON)).0,
        ).unit();
    }

    // refactor vars declared around loops
    let hit = false;
    let depth = 0.0;

    for step in 0..MAX_STEPS {
        let (distance, material) = sdf(ray.point_at(depth));

        // rewrite assignments to match this
        depth += distance;

        if distance <= EPSILON {
            hit = true;
            break;
        }

        if distance >= MAX_DEPTH {
            break;
        }
    }

    // nothing was hit :(
    if !hit {
        // material blank or material sky?
        return CastResult::worst();
    }

    // quick normal estimation
    let normal = normal(ray.point_at(depth));

    // we hit it, here it is!
    return CastResult::new(true, distance, normal, material);
}

fn cast_ray(scene: Scene, ray: Ray) -> CastResult {
    let march = hit_march(scene.march, ray);
    let trace = hit_trace(scene.trace, ray);

    // nothing was hit, so return the sky
    if !march.hit && !trace.hit {
        return CastResult::worst();
    }

    if  trace_hit && !march_hit || trace_dist <= march_dist {
        return trace;
    }

    return march;
}

fn sample_sphere() -> Vec3 {
    // speed up with `let mut rng = rand::thread_rng()` in closure?

    let mut rng = rand::thread_rng();
    let mut point: Vec3;

    while point.length_squared() >= 1.0 {
        point = Vec3::new(
            rng.gen() * 2.0 - 1.0,
            rng.gen() * 2.0 - 1.0,
            rng.gen() * 2.0 - 1.0,
         );
    }

    return point;
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - 2.0 * v.dot(&n) * n;
}

fn fresnel(cosine: f64, ri: f64) -> f64 {
    let mut r0: f64 = (1.0 - ri)/(1.0 + ri);
    r0 = r0*r0;
    return r0 + (1.0-r0)*(1.0-cosine).powi(5);
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64, refracted: &mut Vec3) -> bool {
    let uv: Vec3 = v.unit();
    let dt: f64 = uv.dot(&n);

    let discriminant: f64 = 1.0 - ni_over_nt*ni_over_nt*(1.0 - dt*dt);
    if discriminant > 0.0 {
        *refracted = ni_over_nt*(uv - *n*dt) - *n*((discriminant).sqrt());
        return true;
    } else {
        return false;
    }
}

fn color(scene: Scene, ray: Ray, bounce: u32) -> Vec3 {
    let (hit, distance, normal, material) = cast_ray(scene, ray).unpack();

    // return the sky
    if !hit || bounce = 0 {
        return material.color * material.emission;
    }

    // time for some recursion...
    let position = ray.point_at(distance);

    // we need to get 3 things:
    // a reflection, a diffuse, and a transmission
    let mut diffuse = Vec3::new(0.0, 0.0, 0.0);
    let mut scatter: Ray;

    // diffuse:
    for _ in 0..SAMPLES {
        scatter = Ray::through(position, position + sample_sphere() * material.roughness);
        sample = color(scene, scatter, bounce);

        // lambert thing
        diffuse += (scatter.dot(normal) * sample);
    }

    let mut specular = Vec3::new(0.0, 0.0, 0.0);

    // reflection
    for _ in 0..SAMPLES {
        scatter = Ray::through(
            position,
            position + reflect(ray.direction, normal) + sample_sphere() * material.roughness
        );
        sample = color(scene, scatter, bounce);

        specular += sample;
    }

    let mut transmission = Vec3::new(0.0, 0.0, 0.0);

    // TODO: glass
    for _ in 0..SAMPLES {
        scatter;
    }

    // TODO: fresnel

    diffuse = diffuse / SAMPLES;
    specular = specular / SAMPLES;

    let mut result: Vec3;

    // combine the diffusion and reflection as per metallicness
    // combine the result of the above combination with refraction as per transmission
    // make source emissive as per the emission parameter
    result = (specular * material.metallic) + (diffuse * (1.0 - material.metallic));
    result = (transmission * material.transmission) + (result * (1.0 - material.transmission));
    result = (material.color * material.emission) + (result * (1.0 - material.emission).min(0.0));

    return result;
}

fn make_ray(origin: Vec3, fov: f64, ratio: f64, uv: [f64; 2]) -> Ray {
    // I apologize for this garbage
    let xy = [uv[0] - ratio * 0.5, uv[1] - 0.5];
    let z = 1.0 / (fov.to_radians() / 2.0).tan();
    return Ray::new(origin, (Vec3::new(xy[0], xy[1], -z)).unit());
}

pub fn render(scene: Scene, uv: [f64; 2], resolution: [usize; 2]) -> Vec3 {
    // make ray
    let ray = make_ray(scene.camera.ray.origin, 120.0, (resolution[0] as f64) / (resolution[1] as f64), uv);

    // cast ray
    return color(scene, ray, MAX_BOUNCES);
}
