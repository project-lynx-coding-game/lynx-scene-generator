use image::{ImageBuffer, Rgb};
use std::fs;

use crate::Map;
use crate::map::{Tile, Object};

pub fn render_image(map: &Map, seed: &str) {
    let filepath_floor = format!("./out/{}_floor.png", seed);
    let filepath_objects = format!("./out/{}_objects.png", seed);
    let mut imgbuf_floor = ImageBuffer::new(map.width, map.height);
    for (x, y, pixel) in imgbuf_floor.enumerate_pixels_mut() {
        *pixel = match map.get_tile(x, y) {
            Tile::Grass => Rgb([148 as u8, 204, 71]),
            Tile::Dirt => Rgb([94, 66, 60]),
            Tile::Sand => Rgb([231, 212, 148])
        };
    }

    let mut imgbuf_objects = ImageBuffer::new(map.width, map.height);
    for (x, y, pixel) in imgbuf_objects.enumerate_pixels_mut() {
        *pixel = match map.get_object(x, y) {
            Object::Tree => Rgb([75 as u8, 162, 69]),
            Object::None => Rgb([255, 255, 255])
        };
    }

    fs::create_dir_all("./out").expect("Error creating out dir.");

    match imgbuf_floor.save(&filepath_floor) {
        Ok(_) => println!("Rendered image: {}", filepath_floor),
        Err(e) => eprintln!("Error rendering image {}: {}", filepath_floor, e),
    }

    match imgbuf_objects.save(&filepath_objects) {
        Ok(_) => println!("Rendered image: {}", filepath_objects),
        Err(e) => eprintln!("Error rendering image {}: {}", filepath_objects, e),
    }
}