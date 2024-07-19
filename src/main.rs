use crate::model::game::Game;

pub mod model;

fn main() {
    let game: Option<Game> = None;
    let game = Game::new("game01", 32, 2);

    println!("Hello, world!");
}
