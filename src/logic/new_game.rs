use crate::model::game::{Game, MapCell, PlexType};

pub enum AmountOfPlayers {
    One,
    Four,
    Nine,
    Sixteen,
    TwentyFive,
}

const SPACE_BETWEEN_PLAYERS: u8 = 2;

// known starting formation:
//     h1
//   h1h6h1
//     h1
const STARTING_PATTERN_WIDTH: u8 = 3;
const STARTING_PATTERN_HEIGHT: u8 = 3;

// space to other players means how many empty rows are to other players
pub fn create_new_game_with_initial_setup(
    game_id: &str,
    amount_of_players: AmountOfPlayers,
) -> Game {
    let players = map_enum_to_int(amount_of_players);

    // Our player setup looks like and map is continuous
    //     h1
    //   h1h6h1
    //     h1

    // As spacing between players is 's' and map is continuous then
    // we can think that players share half of spacing. But on the other
    // hand there are spacing on both sides, so SPACE_BETWEEN_PLAYERS / 2 * 2
    // can be simplified
    let min_player_setup_size = STARTING_PATTERN_WIDTH + SPACE_BETWEEN_PLAYERS;

    let sqrt_players = (players as f64).sqrt() as u8;
    let row_size = sqrt_players * min_player_setup_size;

    let game = Game::new(game_id, row_size, players);

    let player_start_points = distribute_players(row_size as usize, players as usize);

    let mut map = game.map.clone();

    // TODO verify that player_num is found from player list. Should include game.validate() ?

    // TODO populate game data
    for (player_num, x_usize, y_usize) in &player_start_points {
        let x: u8 = *x_usize;
        let y: u8 = *y_usize;
        let player_id = *player_num as u8;

        // DEBUG
        //println!("x: {}, y: {}", x, y);

        let cell_s1 = MapCell {
            size: 1,
            plex_type: PlexType::Normal,
            owner: Some(player_id),
        };

        let cell_s6 = MapCell {
            size: 6,
            plex_type: PlexType::Normal,
            owner: Some(player_id),
        };

        map.set(x, y - 1, cell_s1.clone());
        map.set(x - 1, y, cell_s1.clone());
        map.set(x, y, cell_s6.clone());
        map.set(x + 1, y, cell_s1.clone());
        map.set(x, y + 1, cell_s1.clone());
    }

    game.copy(map)
    // TODO randomize generators
}

fn distribute_players(row_size: usize, num_players: usize) -> Vec<(usize, u8, u8)> {
    let mut players_positions = Vec::new();

    // coordinates start from 0,0
    let initial_x = STARTING_PATTERN_WIDTH / 2 + SPACE_BETWEEN_PLAYERS / 2; // TODO what if not even?
    let initial_y = STARTING_PATTERN_HEIGHT / 2 + SPACE_BETWEEN_PLAYERS / 2;

    let mut px = initial_x;
    let mut py = initial_y;

    // TODO instead of iterating we could get ids from somewhere
    for players_num in 1..(num_players + 1) {
        players_positions.push((players_num, px, py));
        px += STARTING_PATTERN_WIDTH + SPACE_BETWEEN_PLAYERS;
        if px >= row_size as u8 {
            py += STARTING_PATTERN_HEIGHT + SPACE_BETWEEN_PLAYERS;
            px = initial_x;
        }
    }

    players_positions
}

// To keep map a rectangle of same number of rows and columns, we have
// enum to define allowed number of players. Following func convert enum to
// integer number.
fn map_enum_to_int(amount_of_players: AmountOfPlayers) -> u8 {
    match amount_of_players {
        AmountOfPlayers::One => 1,
        AmountOfPlayers::Four => 4,
        AmountOfPlayers::Nine => 9,
        AmountOfPlayers::Sixteen => 16,
        AmountOfPlayers::TwentyFive => 25,
    }
}
