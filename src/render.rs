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

// constants
pub const MAX_BOUNCES: u32 = 3;
pub const BRANCH: u32 = 1; // for tree-based path-tracing
pub const EPSILON: f64 = 0.002;
pub const AA: u32 = 8;

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
fn color(scene: &Scene, ray: Ray, bounce: u32, branches: u32) -> Vec3 {
    let (distance, normal, material) = match cast_ray(&scene, ray) {
        Some(cast) if bounce != 0 => (cast.distance, cast.normal, cast.material),
        // hit the sky or traced for too long
        Some(_) => return Vec3::new(0.0, 0.0, 0.0),
        None => return scene.bg.color * scene.bg.emission,
    };

    // let d = (distance - 5.7) / 2.0;
    // return Vec3::new(d, d, d);
    return (normal + 1.0) * 0.5;

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

    return pbr(material, transmission, diffuse, specular);
}

// combine samples in a PBR manner
pub fn pbr(
    material: Material,
    transmission: Vec3,
    diffuse: Vec3,
    specular: Vec3,
) -> Vec3 {
    // TODO: transmission

    // mix transparent and diffuse
    let base = (transmission * material.transmission) + (diffuse * (1.0 - material.transmission));

    // TODO: specular seems off, violating cons. of energy. review.
    let dielectric = base + (specular * material.specular); // with a specular layer on top
    let electric = specular * material.color; // for metallic materials

    // lerp electric and dielectric
    let non_emmisive = (electric * material.metallic) + (dielectric * (1.0 - material.metallic));
    let combined = (material.color * material.emission) + (non_emmisive * (1.0 - material.emission).max(0.0));

    // final color.
    return combined;
}

pub fn sample(
    scene: &Scene,
    camera: &Camera,
    u: f64, v: f64
) -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut aliased = Vec3::new(0.0, 0.0, 0.0);
    let mut sample = 1;

    for _s in 0..AA {
        // shake pixel around
        let (x, y) = (u + rng.gen::<f64>(), v + rng.gen::<f64>());
        let ray = camera.make_ray(x, y);

        let c = color(&scene, ray, MAX_BOUNCES, BRANCH);

        let so_far = (aliased + c) / sample as f64;
        let previous = aliased / ((sample - 1) as f64 + EPSILON);
        let change = (so_far - previous).length();

        if change < 0.001 && sample > (AA / 2) {
            // println!("{}", sample);
            return so_far;
        }

        // cast ray
        aliased = aliased + c;
        sample += 1;
    }

    return aliased / (AA as f64);
}
