mod vec3;
mod write;
mod scene;

use vec3::Vec3;

const RESOLUTION: [usize; 2] = [100, 200];
const RENDER_OUT: &str = "~/Desktop/render.png";

fn main() {

    let image: Vec<Vec<Vec3>>;

    for x in 0..RESOLUTION[0] {
        let row = vec![];

        for y in 0..RESOLUTION[1] {
            row.push(scene::calc_color(
                x / RESOLUTION[0],
                y / RESOLUTION[1],
            ));
        }

        image.push(row);
    }

    write::png(image, RENDER_OUT.to_string());
}

// TODO: write tests
