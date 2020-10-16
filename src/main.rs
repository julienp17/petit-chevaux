extern crate colored;
mod game;

use colored::*;

const HORSE_SYMBOL: &str = "H";
const NB_SQUARES: i8 = 40;

fn main() {
    print_board();
}

fn print_board() {
    let colors = [
        Colorize::on_red("[H]"),
        Colorize::on_yellow("[H]"),
        Colorize::on_green("[H]"),
        Colorize::on_blue("[H]"),
    ];
    let mut color_index = 0;

    for square in 0..NB_SQUARES {
        if is_starting_square(square) {
            print!("{}", colors[color_index]);
            color_index += 1;
        } else {
            print!("[ ]");
        }
    }
    println!();
}

fn is_starting_square(square_number: i8) -> bool {
    return (square_number - 1) % 10 == 0;
}
