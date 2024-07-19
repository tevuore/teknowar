#[cfg(test)]
mod map_tests {
    use crate::model::game::Game;

    #[test]
    fn should_create_empty_map() {
        let game: Option<Game> = None;
        let game = Game::new("game01", 32);

        // access various stuff
        assert_eq!("game01", game.id);
    }
}
