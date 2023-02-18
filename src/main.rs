mod generator;
mod map;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rand;


use generator::Generator;
use map::Map;

use clap::{arg, Command, Arg};

fn main() {
    let matches = Command::new("Lynx Scene Generator")
        .version("1.0")
        .author("Lynx Developers")
        .about("Generates the map")
        .arg(Arg::new("width"))
        .arg(Arg::new("height"))
        .arg(arg!(--seed <VALUE>).required(false))
        .get_matches();

    let width = matches.get_one::<String>("width").expect("Width not specified.");
    let height = matches.get_one::<String>("height").expect("Height not specified.");

    let generator = match matches.get_one::<String>("seed") {
        Some(seed) => {
            let mut hasher = DefaultHasher::new();
            seed.hash(&mut hasher);
            Generator::new(hasher.finish())
        },
        None => {
            Generator::new(rand::random::<u64>())
        }
    };
    let map = Map::new(width.parse::<u32>().unwrap(), height.parse::<u32>().unwrap());
    generator.generate(&map);
    println!("{:?}", map); // TODO add outputting it in a proper format
}