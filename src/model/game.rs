use std::collections::HashMap;

type PlayerId = u8;
type PlayerSymbol = char;

#[derive(Debug, Clone)]
pub struct Game {
    pub id: String, // like game_01
    pub turn: u32,
    pub map: Map,
    pub players: HashMap<PlayerId, Player>,
}

impl Game {
    pub fn new(id: &str, map_size: u8, players: u8) -> Game {
        // TODO minimum map size vs players, check + own func for creating (default spacing)
        Game {
            id: id.to_string(),
            turn: 1,
            map: Map::new(map_size),
            players: Game::create_players(players),
        }
    }

    fn create_players(players: u8) -> HashMap<PlayerId, Player> {
        let mut players_map: HashMap<PlayerId, Player> = HashMap::new();
        let default_energy = 100;

        for i in 0..players {
            let id: PlayerId = i + 1u8;
            let default_name = format!("Player {}", i + 1);
            let symbol_idx = b'A' + i;
            // TODO error if out of symbol range -> return result
            if symbol_idx > b'z' {
                panic!("Available symbols exceeded");
            }
            let symbol: PlayerSymbol = symbol_idx as char; // Use ASCII letters starting from 'A'
            let player = Player {
                id,
                symbol,
                name: default_name,
                energy: default_energy,
            };

            players_map.insert(id, player);
        }

        players_map
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

    // TODO top left corner 1,1 or 0,0 ?
    pub fn get(&self, row: u8, col: u8) -> &MapCell {
        &self.cells[row as usize][col as usize]
    }
}

#[derive(Debug, Clone)]
pub struct MapCell {
    pub size: u8,
    pub owner: Option<PlayerId>, // TODO is this id?
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
    pub symbol: PlayerSymbol,
    pub name: String,
    pub energy: u32,
}
