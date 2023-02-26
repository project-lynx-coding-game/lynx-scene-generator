use serde::Serialize;
use rand_derive2::RandGen;
use std::fmt;

#[derive(Clone, Copy, Debug, Serialize, RandGen, PartialEq)]
pub enum Tile {
    Grass,
    Dirt,
    Sand
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq)]
pub enum Object {
    Tree,
    None
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
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
        if x >= self.width || y >= self.height || self.objects[y as usize][x as usize] == Object::None {
            return None;
        }
        Some(self.objects[y as usize][x as usize])
    }

    pub fn set_object(&mut self, x: u32, y: u32, obj: Object) {
        self.objects[y as usize][x as usize] = obj;
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut id = 0;
        write!(f, "{{\n")?;
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{{\"id\":{}, \"name\":\"{}\", \"position\":{{ \"x\":{}, \"y\":{} }}, \"additional_positions\":[], \"state\":\"\", \"walkable\":\"true\", \"tick\":\"\", \"on_death\":\"\" }},\n",
                        id, self.floor[y as usize][x as usize], x, y)?;
                id += 1;
                if let Some(obj) = self.get_object(x, y) {
                    write!(f, "{{\"id\":{}, \"name\":\"{}\", \"position\":{{ \"x\":{}, \"y\":{} }}, \"additional_positions\":[], \"state\":\"\", \"walkable\":\"true\", \"tick\":\"\", \"on_death\":\"\" }},\n",
                        id, obj, x, y)?;
                    id += 1;
                }
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}