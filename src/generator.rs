use crate::Map;
use crate::map::{Tile, Object};

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

#[derive(Clone)]
struct Point {
    x: u32,
    y: u32
}

impl Point {
    pub fn new(x: u32, y:u32) -> Self {
        Point { x, y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
pub struct Generator {
    seed: u64
}

impl Generator {
    pub fn new(seed: u64) -> Self {
        Generator { seed }
    }


    pub fn generate(&self, map: &mut Map) {
        // Generation method inspired by https://www.procjam.com/tutorials/en/ooze/
        // The approach is to generate randomly placed blobs that grow
        // Based on lazy flood fill algorithm https://youtu.be/YS0MTrjxGbM

        //TODO refactor common parts
        self.generate_floor(map);
        self.generate_trees(map);
    }

    fn generate_trees(&self, map: &mut Map) {
        //TODO ideally add generic flood fill generation
        // tried and its a lot of work, so not sure if worth it
        const DECAY_FACTOR: f32 = 0.5;

        let mut grass_biome = Vec::new();
        for y in 0..map.height {
            for x in 0..map.width {
                if map.get_tile(x, y).unwrap() == Tile::Grass {
                    grass_biome.push(Point::new(x, y));
                }
            }
        }

        let blob_count: u32 = grass_biome.len() as u32 / 10;
        let mut rng = ChaCha8Rng::seed_from_u64(self.seed);

        for _ in 0..blob_count {
            let initial_position = &grass_biome[rng.gen_range(0..grass_biome.len())];
            let mut queue = vec![initial_position.clone()];
            let mut visited = vec![initial_position.clone()];

            let mut chance: f32 = 100.0;

            // loop for growing
            loop {
                if queue.len() == 0 {
                    break;
                }

                //pick random from queue
                let index = rng.gen_range(0..queue.len());
                let point = queue[index].clone();
                queue.remove(index);

                map.set_object(point.x % map.width, point.y % map.height, Object::Tree);

                // chance for neighbours
                if (rng.gen_range(0..=100) as f32) > chance {
                    continue;
                }

                //get neighbours
                let mut neighbours = Vec::new();

                if let Some(Tile::Grass) = map.get_tile(point.x + 1, point.y) {
                    neighbours.push(Point::new(point.x + 1, point.y));
                }
                if point.x > 0 {
                    if let Some(Tile::Grass) = map.get_tile(point.x - 1, point.y) {
                        neighbours.push(Point::new(point.x - 1, point.y));
                    }
                }
                if let Some(Tile::Grass) = map.get_tile(point.x, point.y + 1) {
                    neighbours.push(Point::new(point.x, point.y + 1));
                }
                if point.y > 0 {
                    if let Some(Tile::Grass) = map.get_tile(point.x, point.y - 1) {
                        neighbours.push(Point::new(point.x, point.y - 1));
                    }
                }

                for neighbour in neighbours.iter() {
                    if visited.contains(neighbour) {
                        continue;
                    }

                    visited.push(neighbour.clone());
                    queue.push(neighbour.clone());
                }

                chance *= DECAY_FACTOR;
            }
        }
    }

    fn generate_floor(&self, map: &mut Map) {
        const DECAY_FACTOR: f32 = 0.99;

        let blob_count: u32 = map.width + map.height;
        let mut rng = ChaCha8Rng::seed_from_u64(self.seed);

        for _ in 0..blob_count {
            let blob_type: Tile = rng.gen();

            // initial position
            let x = rng.gen_range(0..map.width);
            let y = rng.gen_range(0..map.height);
            let mut queue = vec![Point::new(x, y)];
            let mut visited = vec![Point::new(x, y)];

            let mut chance: f32 = 100.0;

            // loop for growing
            loop {
                if queue.len() == 0 {
                    break;
                }

                //pick random from queue
                let index = rng.gen_range(0..queue.len());
                let point = queue[index].clone();
                queue.remove(index);

                //set tile on map
                map.set_tile(point.x % map.width, point.y % map.height, blob_type);

                // chance for neighbours
                if (rng.gen_range(0..=100) as f32) > chance {
                    continue;
                }

                //get neighbours
                let mut neighbours = vec![Point::new(point.x + 1, point.y),
                                      Point::new(point.x, point.y + 1)];
                if point.x > 0 {
                    neighbours.push(Point::new(point.x - 1, point.y));
                }
                if point.y > 0 {
                    neighbours.push(Point::new(point.x, point.y - 1));
                }

                for neighbour in neighbours.iter() {
                    if visited.contains(neighbour) {
                        continue;
                    }

                    visited.push(neighbour.clone());
                    queue.push(neighbour.clone());
                }

                chance *= DECAY_FACTOR;
            }
        }
    }
}