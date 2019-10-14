mod vec3::Vec3;
mod ray::Ray;

use f64::consts::PI;

struct CastResult {
    pub distance: f64,
    pub color: Vec3,
    pub normal: Vec3,

}

fn make_ray(fov: f64, ratio: f64, uv: [f64, 2]) -> Ray {
    let xy = [uv[0] - ratio * 0.5, uv[1] - 0.5];
    let z = 1.0 / (FOV.to_radians() / 2.0).tan();
    return (Vec3::new(xy[0], xy[1], -z)).unit();
}

fn cast_ray(ray: Vec3, marchable: Vec<>, traceable: Vec<>, depth: u32) {

    // trace all traceables
    for hittable in traceable {
    }

    // march all marchables

}

fn render(marchable: Vec<>, traceable: Vec<>, uv: [f64; 2], resolution: [usize; 2]) {
    // make ray
    let ray = make_ray(120.0, resolution[0] / resolution[1], uv);

    let distance, normal, color,

    // calculate intersection

    // get the material

    // use the material to calculate:
    // -
}
