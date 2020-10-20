extern crate colored;

use colored::*;
const NB_CELLS: usize = 40;

pub struct Game {
    board: [Cell; NB_CELLS],
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
enum Cell {
    EMPTY = 0,
    RED = 1,
    YELLOW = 2,
    GREEN = 3,
    BLUE = 4,
}

impl Game {
    pub fn create() -> Game {
        let game_board: [Cell; NB_CELLS] = [Cell::EMPTY; NB_CELLS];
        Game { board: game_board }
    }

    fn place_horse(&mut self, horse_color: Cell) {
        let index = 0;
        self.board[index] = horse_color;
    }

    fn move_horse(&mut self, index: usize, to_advance: usize) {
        let horse_number = self.board[index];
        self.board[index] = Cell::EMPTY;
        self.board[(index + to_advance) % NB_CELLS] = horse_number;
    }

    pub fn print_board(self) {
        let colors = [
            Colorize::on_red("[H]"),
            Colorize::on_yellow("[H]"),
            Colorize::on_green("[H]"),
            Colorize::on_blue("[H]"),
        ];
        let mut color_index = 0;

        for square in 0..NB_CELLS {
            if is_start_square(square) {
                print!("{}", colors[color_index]);
                color_index += 1;
            } else {
                print!("[ ]");
            }
        }
        println!();
    }
}

fn is_start_square(square_number: usize) -> bool {
    square_number % 10 == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Color;

    #[test]
    fn can_create_game() {
        let _: Game;
        let _: Game = Game::create();
    }

    #[test]
    fn board_is_initialized() {
        let new_game = Game::create();
        for i in 0..NB_CELLS {
            assert_eq!(new_game.board[i], Cell::EMPTY);
        }
    }

    #[test]
    fn can_place_red_horse() {
        let mut game = Game::create();
        game.place_horse(Cell::RED);
        assert_eq!(game.board[0], Cell::RED);
    }

    #[test]
    fn can_move_horse() {
        let mut game = Game::create();
        game.place_horse(Cell::RED);
        game.move_horse(0, 4);
        assert_eq!(game.board[4], Cell::RED);
    }
}
