use image::{ImageBuffer, Rgb};
use std::fs;

use crate::Map;
use crate::map::{Tile, Object};

pub fn render_image(map: &Map, seed: &str) {
    let filepath = format!("./out/{}.png", seed);
    let mut imgbuf = ImageBuffer::new(map.width, map.height);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = match map.get_tile(x, y) {
            Some(Tile::Grass) => {
                if let Some(Object::Tree) = map.get_object(x, y){
                    Rgb([75 as u8, 162, 69])
                } else {
                    Rgb([148 as u8, 204, 71])
                }
            },
            Some(Tile::Dirt) => Rgb([94, 66, 60]),
            Some(Tile::Sand) => Rgb([231, 212, 148]),
            None => Rgb([0, 0, 0])
        };
    }

    fs::create_dir_all("./out").expect("Error creating out dir.");

    match imgbuf.save(&filepath) {
        Ok(_) => println!("Rendered image: {}", filepath),
        Err(e) => eprintln!("Error rendering image {}: {}", filepath, e),
    }
}