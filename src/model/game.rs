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
        // assuming indexing starts from 0
        let coord_size = size as usize;
        let data = vec![vec![MapCell::empty(); coord_size]; coord_size];
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

/// Find starting coordinates for players
fn distribute_players(
    map_width: usize,
    map_height: usize,
    num_players: usize,
) -> Vec<(usize, usize)> {
    let total_cells = map_width * map_height;
    let spacing = (total_cells as f64 / num_players as f64).sqrt().round() as usize;

    let mut players_positions = Vec::new();
    let mut x = 0;
    let mut y = 0;

    for _ in 0..num_players {
        players_positions.push((x, y));
        x += spacing;
        if x >= map_width {
            x %= map_width;
            y += spacing;
            if y >= map_height {
                y %= map_height;
            }
        }
    }

    players_positions
}
