extern crate colored;

use colored::*;
const NB_SQUARES: usize = 40;

pub struct Game {
    board: [i8; NB_SQUARES],
}

impl Game {
    pub fn create() -> Game {
        let game_board: [i8; NB_SQUARES] = [0; NB_SQUARES];
        Game { board: game_board }
    }

    pub fn print_board(self) {
        let colors = [
            Colorize::on_red("[H]"),
            Colorize::on_yellow("[H]"),
            Colorize::on_green("[H]"),
            Colorize::on_blue("[H]"),
        ];
        let mut color_index = 0;

        for square in 0..NB_SQUARES {
            if is_print_starting_square(square) {
                print!("{}", colors[color_index]);
                color_index += 1;
            } else {
                print!("[ ]");
            }
        }
        println!();
    }
}

fn is_print_starting_square(square_number: usize) -> bool {
    return square_number % 10 == 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_game() {
        let _: Game;
        let _: Game = Game::create();
    }

    #[test]
    fn board_is_initialized() {
        let game = Game::create();

        for i in 0..NB_SQUARES {
            assert_eq!(game.board[i], 0);
        }
    }
}
