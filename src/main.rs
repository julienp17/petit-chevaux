mod game;
use crate::game::Game;

fn main() {
    let mut my_game = Game::new();
    my_game.run();
}
