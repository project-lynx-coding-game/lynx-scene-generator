use rand_derive2::RandGen;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Copy, Debug, Serialize, RandGen, PartialEq)]
pub enum Tile {
    Grass,
    Dirt,
    Sand,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq)]
pub enum Object {
    Tree,
    None,
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
    objects: Vec<Vec<Object>>,
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
            objects,
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
        if x >= self.width
            || y >= self.height
            || self.objects[y as usize][x as usize] == Object::None
        {
            return None;
        }
        Some(self.objects[y as usize][x as usize])
    }

    pub fn set_object(&mut self, x: u32, y: u32, obj: Object) {
        self.objects[y as usize][x as usize] = obj;
    }
}

#[derive(Clone, Debug, Serialize)]
struct Vector {
    x: i32,
    y: i32,
}

#[derive(Debug, Serialize)]
struct Attributes {
    id: i64,
    name: String,
    position: Vector,
    additional_positions: Vec<Vector>,
    state: String,
    walkable: bool,
    tick: String,
    on_death: String,
    owner: String,
    pickable: bool,
    pushable: bool,
}

#[derive(Debug, Serialize)]
struct Entity {
    #[serde(rename = "type")]
    t: String,
    attributes: Attributes,
}

#[derive(Debug, Serialize)]
struct Scene {
    entities: Vec<Entity>,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut id = 0;

        let mut entities: Vec<Entity> = vec![];
        for (x, y) in (0..self.height).zip(0..self.width) {
            entities.push(Entity {
                t: "Object".to_owned(),
                attributes: Attributes {
                    id: id,
                    name: self.floor[y as usize][x as usize].to_string(),
                    position: Vector {
                        x: x as i32,
                        y: y as i32,
                    },
                    additional_positions: vec![],
                    state: "".to_owned(),
                    walkable: true,
                    tick: "".to_owned(),
                    on_death: "".to_owned(),
                    owner: "scene".to_owned(),
                    pickable: false,
                    pushable: false,
                },
            });
            id += 1;
            if let Some(object) = self.get_object(x, y) {
                entities.push(Entity {
                    t: "Object".to_owned(),
                    attributes: Attributes {
                        id: id,
                        name: object.to_string(),
                        position: Vector {
                            x: x as i32,
                            y: y as i32,
                        },
                        additional_positions: vec![],
                        state: "".to_owned(),
                        walkable: false,
                        tick: "".to_owned(),
                        on_death: "".to_owned(),
                        owner: "scene".to_owned(),
                        pickable: false,
                        pushable: false,
                    },
                });
                id += 1;
            }
        }

        let serialized = Scene { entities: entities };
        write!(f, "{}", serde_json::to_string(&serialized).unwrap())?;
        Ok(())
    }
}
