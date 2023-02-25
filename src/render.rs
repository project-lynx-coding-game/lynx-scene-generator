#[cfg(feature = "rendering-images")]
use image::{ImageBuffer, Rgb};
use std::fs;

use crate::Map;
use crate::map::Tile;

pub fn render_image(map: &Map, seed: &str) {
    let filepath = format!("./out/{}.png", seed);
    let mut imgbuf = ImageBuffer::new(map.width, map.height);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = match map.get_tile(x, y) {
            Tile::Grass => Rgb([148 as u8, 204, 71]),
            Tile::Dirt => Rgb([94, 66, 60]),
            Tile::Sand => Rgb([231, 212, 148])
        };
    }

    fs::create_dir_all("./out").expect("Error creating out dir.");

    match imgbuf.save(&filepath) {
        Ok(_) => println!("Rendered image: {}", filepath),
        Err(e) => eprintln!("Error rendering image {}: {}", filepath, e),
    }
}