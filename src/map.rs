use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
pub enum Tile {
    Grass,
    Dirt,
    Sand
}

#[derive(Clone, Copy, Debug, Serialize)]
pub enum Object {
    Tree,
    None
}

#[derive(Debug, Serialize)]
pub struct Map {
    width: u32,
    height: u32,
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
}