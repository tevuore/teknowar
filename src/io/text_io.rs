use crate::model::game::Game;

// TODO fn to center map on player

pub fn convert_to_text_map(game: Game) -> String {
    let map = &game.map;
    let mut buffer = String::new();
    let x_header = generate_x_coords_header(map.size);
    buffer.push_str(&x_header);

    for y in 0..map.size {
        buffer.push_str(&generate_map_row(y, &game))
    }
    buffer
}

/// Generates a row like
/// 01 02 03 04 05 06 07 08 09 10
///
fn generate_x_coords_header(size: u8) -> String {
    let mut buffer = String::new();

    // TODO do we need guard against max size by Result?
    if size >= 100 {
        panic!("X coord value 100 or greater")
    }

    // -- first row

    // adjustment so that y coord is aligned in coming rows
    buffer.push_str("   ");

    for x1 in 0..size {
        if x1 < 10 {
            buffer.push('0');
            buffer.push_str(&x1.to_string());
        } else {
            buffer.push_str(&x1.to_string());
        }

        if x1 < 99 {
            buffer.push(' ');
        }
    }
    buffer.push('\n');

    buffer
}

/// Generates two rows like
/// 0 0 0 0 0 0 0 0 0 0 1
/// 0 1 2 3 4 5 6 7 8 9 0
///
fn generate_x_coords_header_2_row_version(size: u8) -> String {
    let mut buffer = String::new();

    // TODO do we need guard against max size by Result?
    if size >= 100 {
        panic!("X coord value 100 or greater")
    }

    // -- first row

    // adjustment so that y coord is aligned in coming rows
    buffer.push_str("   ");

    for x1 in 0..size {
        if x1 < 10 {
            buffer.push('0');
        } else {
            // 10th value
            let i = x1 / 10;
            buffer.push_str(&i.to_string());
        }

        if x1 < 99 {
            buffer.push(' ');
        }
    }
    buffer.push('\n');

    // -- 2nd row

    // adjustment so that y coord is aligned in coming rows
    buffer.push_str("   ");

    for x2 in 0..size {
        let i = x2 % 10;
        buffer.push_str(&i.to_string());

        if x2 < 99 {
            buffer.push(' ');
        }
    }
    buffer.push('\n');

    buffer
}

/// Generates a row like where first two digits are y coord
/// 01 h1 a2 .. .. b2
///
fn generate_map_row(row: u8, game: &Game) -> String {
    let map = &game.map;
    let players = &game.players;
    let max_x = map.size;

    let mut buffer = String::new();

    let row_10th = row / 10;
    let row_1th = row % 10;
    buffer.push_str(&row_10th.to_string());
    buffer.push_str(&row_1th.to_string());
    buffer.push(' ');

    for x in 0..max_x {
        let cell = map.get(row, x);
        match cell.owner {
            Some(m) => {
                let player_symbol = players
                    .get(&m)
                    .unwrap_or_else(|| panic!("No player for id {m}"))
                    .symbol;
                let size = cell.size;
                let cell_str = format!("{player_symbol}{size}");
                buffer.push_str(&cell_str);
            }
            None => {
                buffer.push_str("..");
            }
        }

        if x < map.size - 1 {
            buffer.push(' ');
        }
    }

    buffer.push('\n');

    buffer
}
