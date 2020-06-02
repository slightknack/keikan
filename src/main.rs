mod structures;
mod objects;
mod write;
mod render;
mod make_scene;

use render::render;
use make_scene::make_scene;
use structures::vec3::Vec3;

const RESOLUTION: [usize; 2] = [200, 100];
const RENDER_OUT: &str = "~/Desktop/render.png"; // make this your own path

fn main() {
    let mut image: Vec<Vec<Vec3>> = vec![];
    let scene = make_scene();

    for y in 0..RESOLUTION[1] {
        let mut row = vec![];

        println!("\rrow {} / {} ", y + 1, RESOLUTION[1]);

        for x in 0..RESOLUTION[0] {
            row.push(render(
                &scene,
                [x as f64, (RESOLUTION[1] - y) as f64],
                RESOLUTION,
            ));
        }

        image.push(row);
    }

    write::png(image, RENDER_OUT.to_string());
}

// TODO: write tests
