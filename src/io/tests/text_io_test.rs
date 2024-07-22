#[cfg(test)]
mod text_io_tests {
    use crate::io::text_io::{convert_to_text_map_2w, convert_to_text_map_3w};
    use crate::model::game::Game;

    #[test]
    fn should_create_empty_map_3w() {
        let map_size = 24;
        let size = map_size as usize;

        let game = Game::new("game01", map_size, 0);

        let text_map = convert_to_text_map_3w(game);
        // for debugging
        // TODO option to turn off?
        print!("{}", text_map);

        let lines: Vec<&str> = text_map.lines().collect();

        let first_line_numbers: Vec<&str> = lines[0].split_whitespace().collect();
        assert_eq!(first_line_numbers.len(), size);

        for x1 in 0..size {
            let expected = if x1 < 10 {
                "0".to_string() + &x1.to_string()
            } else {
                x1.to_string()
            };
            assert_eq!(first_line_numbers[x1], expected.to_string());
        }

        // all following lines
        for line_idx in 1..size {
            let line = lines[line_idx];
            let parts: Vec<&str> = line.split(' ').collect();
            let int_value: u8 = parts[0].parse().expect("Not a valid integer");
            let expected = (line_idx - 1) as u8;
            assert_eq!(int_value, expected);

            for x in 1..size {
                // empty
                assert_eq!(parts[x], "..");
            }
        }
    }

    #[test]
    fn should_create_empty_map_2w() {
        let map_size = 24;
        let size = map_size as usize;

        let game = Game::new("game01", map_size, 0);

        let text_map = convert_to_text_map_2w(game);
        // for debugging
        // TODO option to turn off?
        print!("{}", text_map);

        let lines: Vec<&str> = text_map.lines().collect();

        let first_line_numbers: Vec<&str> = lines[0].split_whitespace().collect();
        assert_eq!(first_line_numbers.len(), size);

        for x1 in 0..size {
            let expected = if x1 < 10 { "0" } else { &(x1 / 10).to_string() };

            assert_eq!(first_line_numbers[x1], expected.to_string());
        }

        let parts_2nd_line: Vec<&str> = lines[1].split_whitespace().collect();
        assert_eq!(parts_2nd_line.len(), size);

        // all following rows
        for row in 2..size {
            let line = lines[row];

            // check row number value
            let parts: Vec<&str> = line.split_whitespace().collect();
            let int_value: u8 = parts[0].parse().expect("Not a valid integer");
            let expected: u8 = (row - 2) as u8;
            assert_eq!(int_value, expected);

            // then rest of characters on a row are empty
            for x in 1..size {
                assert_eq!(parts[x], ".");
            }
        }
    }

    #[test]
    fn should_create_small_map_with_players() {
        let game = Game::new("game01", 10, 2);

        let text_map = convert_to_text_map_3w(game);

        // TODO find out players and verify initial formation

        print!("{}", text_map);
    }

    fn should_create_large_map_with_players() {
        // TODO make constants for map sizes
        // TODO make func for maximum players
        // TODO verify spacing

        let game = Game::new("game01", 24, 10);

        let text_map = convert_to_text_map_3w(game);

        print!("{}", text_map);
    }
}
