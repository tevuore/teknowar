#[cfg(test)]
mod text_io_tests {
    use crate::logic::new_game::{create_new_game_with_initial_setup, AmountOfPlayers};
    use crate::storage::text_io::convert_to_text_map_3w;

    #[test]
    fn should_calculate_initial_positions() {}

    #[test]
    fn should_create_game_with_1_player() {
        // one player game is used just for testing some features
        let game = create_new_game_with_initial_setup("game01", AmountOfPlayers::One);

        let text_map = convert_to_text_map_3w(game);

        // for debugging
        // TODO option to turn off?
        print!("{}", text_map);

        // TODO find out players and verify initial formation
    }

    #[test]
    fn should_create_game_with_4_players() {
        let game = create_new_game_with_initial_setup("game01", AmountOfPlayers::Four);

        let text_map = convert_to_text_map_3w(game);

        // for debugging
        // TODO option to turn off?
        print!("{}", text_map);

        // TODO find out players and verify initial formation
    }

    #[test]
    fn should_create_game_with_9_players() {
        let game = create_new_game_with_initial_setup("game01", AmountOfPlayers::Nine);

        let text_map = convert_to_text_map_3w(game);

        // for debugging
        // TODO option to turn off?
        print!("{}", text_map);

        // TODO find out players and verify initial formation
    }

    #[test]
    fn should_create_game_with_16_players() {
        let game = create_new_game_with_initial_setup("game01", AmountOfPlayers::Sixteen);

        let text_map = convert_to_text_map_3w(game);

        // for debugging
        // TODO option to turn off?
        print!("{}", text_map);

        // TODO find out players and verify initial formation
    }

    #[test]
    fn should_create_game_with_25_players() {
        let game = create_new_game_with_initial_setup("game01", AmountOfPlayers::TwentyFive);

        let text_map = convert_to_text_map_3w(game);

        // for debugging
        // TODO option to turn off?
        print!("{}", text_map);

        // TODO find out players and verify initial formation
    }
}
