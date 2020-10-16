mod game;
use crate::game::Game;

fn main() {
    let my_game = Game::create();
    my_game.print_board();
}
