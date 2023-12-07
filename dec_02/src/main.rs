pub mod error;
mod game;

use crate::{
    game::grab::Grab,
    game::Game
};
use bool_ext::BoolExt;
use error::Result;
use std::{
    fs::read_to_string,
    path::PathBuf
};

const INPUT_FILE_PATH: &str = "/Users/avasurus/advent_of_code_2023/dec_02/input.txt";

fn main() -> Result<()> {
    let max_grab = Grab::new(12, 14, 13);

    // Part 1:
    let mut res = 0;

    for line in read_to_string::<PathBuf>(INPUT_FILE_PATH.into())?.lines() {
        let game = Game::parse(line)?;
        res += game.is_valid_game(max_grab).map_or_else(|| 0, || game.id());
    }

    println!("Valid game ID sum: {res}");


    let mut res = 0;

    for line in read_to_string::<PathBuf>(INPUT_FILE_PATH.into())?.lines() {
        let game = Game::parse(line)?;
        let min_grab = game.min_valid_grab();
        res += min_grab.red_count() * min_grab.blue_count() * min_grab.green_count();
    }

    println!("Min cube power set sum: {res}");
    Ok(())
}
