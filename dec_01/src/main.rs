mod calibrate;
mod error;

use crate::calibrate::ICalibrateExt;
use error::Result;
use std::{fs::read_to_string, path::PathBuf};

const INPUT_FILE_PATH: &str = "/Users/avasurus/advent_of_code_2023/dec_01/input.txt";

fn main() -> Result<()> {
    let mut sum = 0;

    for line in read_to_string::<PathBuf>(INPUT_FILE_PATH.into())?.lines() {
        sum += line.calibrate()?;
    }

    println!("Calibration sum: {sum}");

    Ok(())
}
