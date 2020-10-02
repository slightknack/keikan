use std::env;
use std::path::Path;
// use tokio::prelude::*;

mod structures;
mod objects;
mod write;
mod render;
mod demo;

// runs the demo
fn main() {
    let output: String = match env::args().nth(1) {
        Some(p) => p,
        None    => {
            eprintln!("Expected output path CLI argument");
            return;
        },
    };

    let (scene, camera) = demo::shadow();
    let image = scene.render(camera);

    match write::png(image, Path::new(&output.to_string())) {
        Ok(())   => (),
        Err(_) => eprintln!("Could not save image!"),
    }
}

// TODO: write tests
