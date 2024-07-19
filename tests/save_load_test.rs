extern crate teknowar;
use crate::teknowar::model::game::Game;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn should_save_and_load_map() {
        let game = Game::new("game01", 32);

        // TODO save

        // TODO load

        assert_eq!("game01", game.id);
    }
}
