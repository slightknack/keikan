use std::path::Path;
// use tokio::prelude::*;

mod structures;
mod objects;
mod write;
mod render;
mod make_scene;

use make_scene::make_scene;

const RENDER_OUT: &str = "/Users/isaac/Desktop/render.png"; // make this your own path

fn main() {
    let scene = make_scene();
    let image = scene.render();

    match write::png(image, Path::new(&RENDER_OUT.to_string())) {
        Ok(())   => (),
        Err(_) => eprintln!("Could not save image!"),
    }
}

// TODO: write tests
