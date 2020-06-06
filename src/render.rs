use std::f64;
use std::sync::Arc;
use rand::Rng;

use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;
use crate::structures::camera::Camera;
use crate::structures::material::Material;
use crate::structures::scene::Scene;
use crate::structures::cast_result::CastResult;
use crate::objects::traits::{ March, Trace };

// constants
const MAX_STEPS: u32 = 128;
const MAX_DEPTH: u32 = 10;
const MAX_BOUNCES: u32 = 5;
const SAMPLES: u32 = 1;
const EPSILON: f64 = 0.002;
const AA: u32 = 1;

// TODO: refactor rendering code into impl for scene, camera, and materials, etc.
// TODO: results are trapped and rays will self-intersect, especially for metals
fn hit_march(march: &Vec<Arc<dyn March>>, ray: Ray) -> CastResult {
    let sdf = |point: Vec3| {
        let mut min = f64::MAX;
        let mut mat = Material::blank();

        for object in march.iter() {
            let distance = object.march(point);

            if distance <= min {
                min = distance;
                mat = object.material();
            }
        }

        return (min, mat);
    };

    let normal = |p: Vec3| {
        Vec3::new(
            sdf(Vec3::new(p.x + EPSILON, p.y, p.z)).0 - sdf(Vec3::new(p.x - EPSILON, p.y, p.z)).0,
            sdf(Vec3::new(p.x, p.y + EPSILON, p.z)).0 - sdf(Vec3::new(p.x, p.y - EPSILON, p.z)).0,
            sdf(Vec3::new(p.x, p.y, p.z + EPSILON)).0 - sdf(Vec3::new(p.x, p.y, p.z - EPSILON)).0,
        ).unit()
    };

    let mut depth = EPSILON;

    for step in 0..MAX_STEPS {
        let point = ray.point_at(&depth);
        let (distance, material) = sdf(point);

        if distance <= EPSILON {
            let normal = normal(point); // quick normal estimation

            // let mut mat = Material::blank();
            // mat.color = normal;

            return CastResult::new(true, depth, normal, material);
        }

        if distance >= MAX_DEPTH.into() {
            break;
        }

        depth += distance;
    }
    return CastResult::worst();
}

fn hit_trace(trace: &Vec<Arc<dyn Trace>>, ray: Ray) -> CastResult {
    let mut best = CastResult::worst();

    for object in trace.iter() {
        let (hit, distance, normal) = object.trace(ray);

        if hit && distance > EPSILON && (best.hit == false || distance <= best.distance) {
            best = CastResult::new(hit, distance, normal, object.material());
        }
    }

    return best;
}

fn cast_ray(scene: &Scene, ray: Ray) -> CastResult {
    let march = hit_march(&scene.march, ray);
    let trace = hit_trace(&scene.trace, ray);

    // nothing was hit, so return the sky
    if !march.hit && !trace.hit {
        return CastResult::worst();
    }

    if  trace.hit && !march.hit || trace.distance <= march.distance {
        return trace;
    }

    return march;
}

fn sample_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut point: Vec3 = Vec3::max();

    // sample point in unit cube, check if in unit sphere
    while point.length_squared() >= 1.0 {
        point = Vec3::new(
            rng.gen::<f64>() * 2.0 - 1.0,
            rng.gen::<f64>() * 2.0 - 1.0,
            rng.gen::<f64>() * 2.0 - 1.0,
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

// simplify
fn color(scene: &Scene, ray: Ray, bounce: u32, samples: u32) -> Vec3 {
    let (hit, distance, normal, material) = cast_ray(&scene, ray).unpack();

    // nothing hit, return the sky
    if !hit || bounce <= 0 {
        return material.color * material.emission;
    }

    // return (normal + 1.0) * 0.5;

    let     position     = ray.point_at(&distance);
    let mut diffuse      = Vec3::new(0.0, 0.0, 0.0);
    let mut specular     = Vec3::new(0.0, 0.0, 0.0);
    let mut transmission = Vec3::new(0.0, 0.0, 0.0);

    // diffuse
    for _ in 0..samples {
        let scatter = Ray::through(position, (normal + sample_sphere()) - position);
        let sample = color(&scene, scatter, (bounce - 1), 1); // (samples / 2).max(1)); // only take one sample

        diffuse = diffuse + material.color * sample;
    }

    diffuse = diffuse / (samples as f64);

    // specular
    if material.roughness == 0.0 {
        let scatter = Ray::new(position, reflect(ray.direction, normal).unit());
        specular = color(&scene, scatter, (bounce - 1), samples);
    } else {
        for _ in 0..samples {
            let scatter = Ray::new(position, reflect(ray.direction, normal).unit());
            // let scatter = Ray::through(
            //     position,
            //     (reflect(ray.direction, normal) + sample_sphere() * material.roughness) - position,
            // );

            let sample = color(&scene, scatter, (bounce - 1), (samples / 2).max(1));
            specular = specular + sample;
        }

        specular = specular / (samples as f64);
    }

    // TODO: transmission

    // combine the samples in a PBR manner
    return (
        (
            ( // for dielectric materials. TODO: fresnel blending
                (
                    (transmission *        material.transmission)  // mix transparent
                  + (diffuse      * (1.0 - material.transmission)) // and diffuse
                )
              + (specular * material.specular) // with a specular layer on top
              // TODO: specular seems off, violating cons. of energy. review.
            )
          * (1.0 - material.metallic) // lerp with metal

          + ( // for metallic materials
                specular * material.color
            )
          * material.metallic
        )
      * (1.0 - material.emission).max(0.0) // modified lerp with emissive

      + ( // for emissive materials
          material.color * material.emission
        )
    );
}

pub fn render(scene: &Scene, uv: (f64, f64)) -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut aliased = Vec3::new(0.0, 0.0, 0.0);

    for _ in 0..AA {
        // shake pixel around
        let xy = (uv.0 + rng.gen::<f64>(), uv.1 + rng.gen::<f64>());
        let ray = scene.camera.make_ray(xy);

        // cast ray
        aliased = aliased + color(&scene, ray, MAX_BOUNCES, SAMPLES);
    }

    return aliased / (AA as f64);
}
