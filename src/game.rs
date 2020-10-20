extern crate colored;

use colored::*;
const NB_CELLS: usize = 40;

pub struct Game {
    board: [Cell; NB_CELLS],
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
enum Cell {
    EMPTY,
    RED,
    YELLOW,
    GREEN,
    BLUE,
}

impl Game {
    pub fn create() -> Game {
        let game_board: [Cell; NB_CELLS] = [Cell::EMPTY; NB_CELLS];
        Game { board: game_board }
    }

    fn place_horse(&mut self, horse_color: Cell) {
        let index = 10 * (horse_color as usize - 1);
        self.board[index] = horse_color;
    }

    fn move_horse(&mut self, index: usize, to_advance: usize) {
        let horse_color = self.board[index];
        if horse_color == Cell::EMPTY {
            return;
        }
        self.board[index] = Cell::EMPTY;
        self.board[(index + to_advance) % NB_CELLS] = horse_color;
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
    fn can_place_yellow_horse() {
        let mut game = Game::create();
        game.place_horse(Cell::YELLOW);
        assert_eq!(game.board[10], Cell::YELLOW);
    }

    #[test]
    fn can_place_green_horse() {
        let mut game = Game::create();
        game.place_horse(Cell::GREEN);
        assert_eq!(game.board[20], Cell::GREEN);
    }

    #[test]
    fn can_place_blue_horse() {
        let mut game = Game::create();
        game.place_horse(Cell::BLUE);
        assert_eq!(game.board[30], Cell::BLUE);
    }

    #[test]
    fn can_move_horse() {
        let mut game = Game::create();
        game.place_horse(Cell::RED);
        game.move_horse(0, 4);
        assert_eq!(game.board[4], Cell::RED);
    }

    #[test]
    fn can_move_horse_after_board_loop() {
        let mut game = Game::create();
        game.place_horse(Cell::BLUE);
        game.move_horse(30, 12);
        assert_eq!(game.board[2], Cell::BLUE);
    }

    #[test]
    fn can_move_horse_no_horse_at_index() {
        let mut game = Game::create();
        game.move_horse(4, 3);
        for &cell in game.board.iter() {
            assert_eq!(cell, Cell::EMPTY);
        }
        game.place_horse(Cell::YELLOW);
        game.move_horse(9, 1);
        assert_eq!(game.board[10], Cell::YELLOW);
    }
}
