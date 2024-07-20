#[cfg(test)]
mod map_tests {
    use crate::model::game::Game;

    #[test]
    fn should_create_empty_map() {
        let game = Game::new("game01", 32, 2);

        assert_eq!("game01", game.id);
        assert_eq!(32, game.map.size);
        assert_eq!(1, game.turn);

        assert_eq!(game.players.len(), 2);

        let id1 = 1u8;
        let id2 = 2u8;

        let player1 = &game.players[&id1];
        let player2 = &game.players[&id2];

        assert!(!player1.name.is_empty());
        assert_ne!(player1.name, player2.name);

        assert_eq!(player1.energy, 100);
        assert_eq!(player2.energy, 100);

        // TODO check that players are found from map
        // TODO later better check for starting pattern
    }

    // TODO test for exceeding max players
}
