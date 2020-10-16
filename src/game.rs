const NB_SQUARES: usize = 40;

struct Game {
    board: [i8; NB_SQUARES],
}

impl Game {
    fn create() -> Game {
        let game_board: [i8; NB_SQUARES] = [0; NB_SQUARES];
        Game { board: game_board }
    }
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
