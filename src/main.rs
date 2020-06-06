use std::path::Path;
// use tokio::prelude::*;

mod structures;
mod objects;
mod write;
mod render;
mod make_scene;

use render::render;
use make_scene::make_scene;
use structures::vec3::Vec3;

const RENDER_OUT: &str = "/Users/isaac/Desktop/render.png"; // make this your own path

fn main() {
    let scene = make_scene();
    let image = scene.render();

    write::png(image, Path::new(&RENDER_OUT.to_string()));
}

// TODO: write tests
