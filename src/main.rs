use crate::model::game::Game;

mod io;
pub mod model;

fn main() {
    let game = Game::new("game01", 32, 2);

    // TODO interpret cli command

    // TODO 'show map and players'
    // TODO 'generate report for single player'

    // TODO 'run turn'
    //  - read data
    //  - perform players actions
    //  - growing phase
    //  - collect energies
    //  - save data to new files

    println!("DONE");
}
