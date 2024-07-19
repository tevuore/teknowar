use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Game {
    pub id: String, // like game_01
    pub turn: u32,
    pub map: Map,
    pub players: HashMap<String, Player>,
}

impl Game {
    pub fn new(id: &str, size: u8) -> Game {
        Game {
            id: id.to_string(),
            turn: 1,
            map: Map::new(size),
            players: HashMap::new(), // TODO create players
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    pub size: u8, // size of rectangle x,x
    pub cells: Vec<Vec<MapCell>>,
}

impl Map {
    pub fn new(size: u8) -> Map {
        let data = vec![vec![MapCell::empty(); size as usize]; size as usize];
        Map { size, cells: data }
    }

    pub fn get(&self, row: u8, col: u8) -> &MapCell {
        &self.cells[row as usize][col as usize]
    }
}

#[derive(Debug, Clone)]
pub struct MapCell {
    pub size: u8,
    pub owner: Option<String>, // TODO is this id?
                               // TODO infection?
}

impl MapCell {
    pub fn empty() -> MapCell {
        MapCell {
            size: 0,
            owner: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub id: u8,
    pub symbol: char,
    pub name: String,
    pub energy: u32,
}
