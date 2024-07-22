extern crate teknowar;
use crate::teknowar::model::game::Game;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn should_save_and_load_map() {
        let game = Game::new("game01", 24u8, 25u8);

        // TODO save

        // TODO load

        assert_eq!("game01", game.id);
        assert_eq!(24, game.map.size);
        assert_eq!(1, game.turn);
        assert_eq!(25, game.players.len());
    }
}
