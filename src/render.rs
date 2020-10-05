use std::f64;
use rand::Rng;

use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;
use crate::structures::material::Material;
use crate::structures::scene::Scene;
use crate::structures::cast::Cast;
use crate::structures::camera::Camera;
use crate::objects::march::March;
use crate::objects::trace::Trace;

pub const EPSILON: f64 = 0.0005;

fn cast_ray(scene: &Scene, ray: Ray) -> Option<Cast> {
    let march = March::hit(&scene.march, ray);
    let trace = Trace::hit(&scene.trace, ray);

    match (march, trace) {
        (None, None) => None,
        (None, t @ Some(_)) => t,
        (m @ Some(_), None) => m,
        // trace results are more exact, so favor in a tie.
        (Some(m), Some(t)) => Some(if m.distance < t.distance { m } else { t }),
    }
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

fn sample_sphere_surface() -> Vec3 {
    sample_sphere().unit()
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - 2.0 * v.dot(&n) * n;
}

// adapted from Casual Shadertoy Path Tracing Part III (demofox.org)
fn fresnel(ior: f64, normal: Vec3, ray: Ray) -> f64 {
    let mut r0: f64 = (1.0 - ior)/(1.0 + ior);
    let cosine = -normal.dot(&ray.direction);
    r0 = r0*r0;
    let unclamped = r0 + (1.0-r0) * (1.0-cosine).powi(5);
    return (0.0 as f64).max(unclamped.min(1.0));
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
fn color(scene: &Scene, ray: Ray, bounce: usize, branches: usize) -> Vec3 {
    let (distance, normal, material) = match cast_ray(&scene, ray) {
        Some(cast) if bounce != 0 => (cast.distance, cast.normal, cast.material),
        // hit the sky or traced for too long
        Some(_) => return Vec3::new(0.0, 0.0, 0.0),
        _ => return scene.bg.color * scene.bg.emission,
    };

    // uncomment to debug depth map:
    // return Vec3::new(1.0/distance, 1.0/distance, 1.0/distance);

    // uncomment to debug normal map:
    // return (normal + 1.0) * 0.5;

    let     position     = ray.point_at(&distance);
    let mut diffuse      = Vec3::new(0.0, 0.0, 0.0);
    let mut specular     = Vec3::new(0.0, 0.0, 0.0);
    let     transmission = Vec3::new(0.0, 0.0, 0.0);

    // diffuse
    for _ in 0..branches {
        let scatter = Ray::new(position, (sample_sphere_surface() + normal).unit());
        let sample = color(&scene, scatter, bounce - 1, 1); // (samples / 2).max(1)); // only take one sample

        diffuse = diffuse + material.color * sample;
    }

    diffuse = diffuse / (branches as f64);

    // specular
    for _ in 0..branches {
        // let scatter = Ray::new(position, reflect(ray.direction, normal).unit());
        let scatter = Ray::new(
            position,
            reflect(ray.direction, normal + (sample_sphere() * material.roughness)),
        );

        let sample = color(&scene, scatter, bounce - 1, (branches / 2).max(1));
        specular = specular + sample;
    }

    specular = specular / (branches as f64);

    // this calculation of IOR looks fine,
    // but specular is defined as a percent,
    // so this might not be correct
    let sqrtm = material.specular.sqrt();
    let ior = (1.0 - sqrtm * 0.28) / (sqrtm * 0.28 + 1.0); // 0.28 is ~ sqrt(0.08)
    let f = fresnel(ior, normal, ray);
    // return Vec3::new(f, f, f);

    return pbr(material, transmission, diffuse, specular, f);
}

// combine samples in a PBR manner
pub fn pbr(
    material: Material,
    transmission: Vec3,
    diffuse: Vec3,
    specular: Vec3,
    fresnel: f64,
) -> Vec3 {
    // TODO: transmission

    // mix transparent and diffuse
    let base = (transmission * material.transmission) + diffuse * (1.0 - material.transmission);

    // with a specular layer on top
    let dielectric = (specular * fresnel) + base * (1.0 - fresnel);
    // for metallic materials
    let electric = specular * material.color;

    // lerp electric and dielectric
    let non_emmisive = (electric * material.metallic) + dielectric * (1.0 - material.metallic);
    let combined = non_emmisive + material.color * material.emission;

    // final color.
    return combined;
}

pub fn sample(
    scene: &Scene,
    camera: &Camera,
    rng: &mut impl Rng,
    u: f64, v: f64
) -> Vec3 {
    let mut aliased = Vec3::new(0.0, 0.0, 0.0);

    for _s in 0..camera.aa {
        // shake pixel around
        let (x, y) = (u + rng.gen::<f64>(), v + rng.gen::<f64>());
        let ray = camera.make_ray(x, y);

        // cast ray
        aliased = aliased + color(&scene, ray, camera.bounces, camera.branch);
    }

    return aliased / (camera.aa as f64);
}
