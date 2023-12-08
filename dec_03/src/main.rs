mod engine_schematic;
pub mod error;

use engine_schematic::EngineSchematic;
use error::Result;
use std::fs::read_to_string;
use std::path::PathBuf;

const INPUT_FILE_PATH: &str = "/Users/avasurus/advent_of_code_2023/dec_03/input.txt";

fn main() -> Result<()> {
    // Part 1
    let symbol_grid =
        EngineSchematic::new(read_to_string::<PathBuf>(INPUT_FILE_PATH.into())?, |c| {
            !(c.is_numeric() || c.eq(&'.'))
        })?;
    let part_numbers_sum = symbol_grid.sum();

    println!("Part numbers sum: {part_numbers_sum}");

    // Part 2
    let gear_grid =
        EngineSchematic::new(read_to_string::<PathBuf>(INPUT_FILE_PATH.into())?, |c| {
            c.eq(&'*')
        })?;
    let ratio_sum = gear_grid.ratio_sum();

    println!("Ratio sum: {ratio_sum}");

    Ok(())
}
