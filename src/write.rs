use image::{ ImageBuffer, Rgb, ImageRgb8 };
use std::path::Path;
use std::io;

use crate::structures::vec3::Vec3;

pub fn png(image: Vec<Vec<Vec3>>, path: &Path) -> io::Result<()> {
    // TODO: error handling

    // new buffer the width and height of the render
    let mut buffer = ImageBuffer::new(
        image[0].len() as u32,
        image.len() as u32,
    );

    for (y, row) in image.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            buffer.put_pixel(x as u32, y as u32, Rgb(pixel.colorize(1.0)));
        }
    }

    ImageRgb8(buffer).save(&path)?;
    println!("Render saved to {}", path.display());
    return Ok(());
}
