use serde::Serialize;
use rand_derive2::RandGen;

#[derive(Clone, Copy, Debug, Serialize, RandGen, PartialEq)]
pub enum Tile {
    Grass,
    Dirt,
    Sand
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq)]
pub enum Object {
    Tree,
    None
}

#[derive(Debug, Serialize)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    floor: Vec<Vec<Tile>>,
    objects: Vec<Vec<Object>>
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        let mut floor = Vec::new();
        let mut objects = Vec::new();
        for _ in 0..height {
            let floor_row = vec![Tile::Grass; width as usize];
            let object_row = vec![Object::None; width as usize];
            floor.push(floor_row);
            objects.push(object_row);
        }

        Map {
            width,
            height,
            floor,
            objects
        }
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Option<Tile> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(self.floor[y as usize][x as usize])
    }

    pub fn set_tile(&mut self, x: u32, y: u32, tile: Tile) {
        self.floor[y as usize][x as usize] = tile;
    }

    pub fn get_object(&self, x: u32, y: u32) -> Option<Object> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(self.objects[y as usize][x as usize])
    }

    pub fn set_object(&mut self, x: u32, y: u32, obj: Object) {
        self.objects[y as usize][x as usize] = obj;
    }
}