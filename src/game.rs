use colored::*;
use rand::Rng;
const NB_CELLS: usize = 40;
const NB_START_HORSES: usize = 2;
const NB_PLAYERS: usize = 4;

pub struct Game {
    board: [Cell; NB_CELLS],
    stables: [usize; NB_PLAYERS],
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
enum Cell {
    RED,
    YELLOW,
    GREEN,
    BLUE,
    EMPTY,
}

impl Game {
    pub fn new() -> Game {
        let game_board: [Cell; NB_CELLS] = [Cell::EMPTY; NB_CELLS];
        let game_stables: [usize; NB_PLAYERS] = [NB_START_HORSES; NB_PLAYERS];
        Game {
            board: game_board,
            stables: game_stables,
        }
    }

    pub fn run(&mut self) {
        self.place_horse(Cell::RED);
        self.place_horse(Cell::YELLOW);
        self.print_board();
        self.move_horse(0, 4);
        self.print_board();
        self.move_horse(4, 6);
        self.print_board();
    }

    fn roll_dice(&self) -> usize {
        rand::thread_rng().gen_range(1, 6 + 1)
    }

    fn cell_is_empty(&self, index: usize) -> bool {
        self.board[index] == Cell::EMPTY
    }

    fn place_horse(&mut self, horse_color: Cell) {
        let start_index = 10 * horse_color as usize;
        if !self.cell_is_empty(start_index) {
            self.kick_horse(start_index)
        }
        self.board[start_index] = horse_color;
        self.stables[horse_color as usize] -= 1;
    }

    fn kick_horse(&mut self, index: usize) {
        if self.cell_is_empty(index) {
            return;
        }
        let kicked_horse_color = self.board[index];
        self.board[index] = Cell::EMPTY;
        self.stables[kicked_horse_color as usize] += 1;
    }

    fn move_horse(&mut self, index: usize, to_advance: usize) {
        if self.cell_is_empty(index) {
            return;
        }
        let horse_color: Cell = self.board[index];
        let new_index = (index + to_advance) % NB_CELLS;
        if !self.cell_is_empty(new_index) {
            self.kick_horse(new_index);
        }
        self.board[index] = Cell::EMPTY;
        self.board[new_index] = horse_color;
    }

    fn print_board(&self) {
        for cell in self.board.iter() {
            match cell {
                Cell::RED => print!("{}", "[H]".on_red()),
                Cell::YELLOW => print!("{}", "[H]".on_yellow()),
                Cell::GREEN => print!("{}", "[H]".on_green()),
                Cell::BLUE => print!("{}", "[H]".on_blue()),
                Cell::EMPTY => print!("[ ]"),
            }
        }
        println!();
    }
}

//////////////////////////// UNIT TESTING ////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_game() {
        let _: Game;
        let _: Game = Game::new();
    }

    #[test]
    fn board_is_initialized() {
        let game = Game::new();
        for &cell in game.board.iter() {
            assert_eq!(cell, Cell::EMPTY);
        }
    }

    #[test]
    fn stables_are_initialized() {
        let game = Game::new();
        for &stable in game.stables.iter() {
            assert_eq!(stable, NB_START_HORSES);
        }
    }

    #[test]
    fn can_place_red_horse() {
        let mut game = Game::new();
        game.place_horse(Cell::RED);
        assert_eq!(game.board[0], Cell::RED);
        assert_eq!(game.stables[Cell::RED as usize], NB_START_HORSES - 1);
    }

    #[test]
    fn can_place_yellow_horse() {
        let mut game = Game::new();
        game.place_horse(Cell::YELLOW);
        assert_eq!(game.board[10], Cell::YELLOW);
        assert_eq!(game.stables[Cell::YELLOW as usize], NB_START_HORSES - 1);
    }

    #[test]
    fn can_place_green_horse() {
        let mut game = Game::new();
        game.place_horse(Cell::GREEN);
        assert_eq!(game.board[20], Cell::GREEN);
        assert_eq!(game.stables[Cell::GREEN as usize], NB_START_HORSES - 1);
    }

    #[test]
    fn can_place_blue_horse() {
        let mut game = Game::new();
        game.place_horse(Cell::BLUE);
        assert_eq!(game.board[30], Cell::BLUE);
        assert_eq!(game.stables[Cell::BLUE as usize], NB_START_HORSES - 1);
    }

    #[test]
    fn place_horse_kick_horse() {
        let mut game = Game::new();
        game.place_horse(Cell::RED);
        game.move_horse(0, 10);
        game.place_horse(Cell::YELLOW);
        assert_eq!(game.board[10], Cell::YELLOW);
        assert_eq!(game.stables[Cell::YELLOW as usize], NB_START_HORSES - 1);
        assert_eq!(game.stables[Cell::RED as usize], NB_START_HORSES);
    }

    #[test]
    fn can_move_horse() {
        let mut game = Game::new();
        game.place_horse(Cell::RED);
        game.move_horse(0, 4);
        assert_eq!(game.board[4], Cell::RED);
    }

    #[test]
    fn can_move_horse_board_loop() {
        let mut game = Game::new();
        game.place_horse(Cell::BLUE);
        game.move_horse(30, 12);
        assert_eq!(game.board[2], Cell::BLUE);
    }

    #[test]
    fn move_horse() {
        let mut game = Game::new();
        game.place_horse(Cell::RED);
        game.move_horse(0, 3);
        assert_eq!(game.board[3], Cell::RED);
    }

    #[test]
    fn move_horse_kick_horse() {
        let mut game = Game::new();
        game.place_horse(Cell::RED);
        game.place_horse(Cell::YELLOW);
        game.move_horse(0, 10);
        assert_eq!(game.board[10], Cell::RED);
        assert_eq!(game.stables[Cell::YELLOW as usize], NB_START_HORSES);
    }

    #[test]
    fn move_horse_empty_board_empty_at_index() {
        let mut game = Game::new();
        game.move_horse(4, 3);
        for &cell in game.board.iter() {
            assert_eq!(cell, Cell::EMPTY);
        }
    }

    #[test]
    fn move_horse_empty_at_index() {
        let mut game = Game::new();
        game.place_horse(Cell::YELLOW);
        game.move_horse(9, 1);
        assert_eq!(game.board[10], Cell::YELLOW);
    }

    #[test]
    fn kick_horse() {
        let mut game = Game::new();
        game.place_horse(Cell::RED);
        game.kick_horse(0);
        assert_eq!(game.board[0], Cell::EMPTY);
        assert_eq!(game.stables[Cell::RED as usize], NB_START_HORSES);
    }

    #[test]
    fn kick_horse_no_horse_at_index() {
        let mut game = Game::new();
        game.kick_horse(0);
        assert_eq!(game.stables[Cell::RED as usize], NB_START_HORSES);
    }
}
