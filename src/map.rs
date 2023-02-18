#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Grass,
    Dirt,
    Sand
}

#[derive(Clone, Copy, Debug)]
pub enum Object {
    Tree,
    None
}

#[derive(Debug)]
pub struct Map {
    width: u32,
    height: u32,
    floor: Vec<Vec<Tile>>,
    objects: Vec<Vec<Object>>
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        let mut floor = Vec::new();
        for _ in 0..height {
            let row = vec![Tile::Grass; width as usize];
            floor.push(row);
        }

        let mut objects = Vec::new();
        for _ in 0..height {
            let row = vec![Object::None; width as usize];
            objects.push(row);
        }

        Map {
            width,
            height,
            floor,
            objects
        }
    }
}