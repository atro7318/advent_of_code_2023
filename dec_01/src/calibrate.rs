#[cfg(test)]
mod numeric_and_word_repr_tests;
#[cfg(test)]
mod numeric_only_tests;
pub mod error;

use error::{Error, Result};
use regex::Regex;

/// Extension trait on a String that returns a two digit number composed of the first instance of a
/// numeric digit in the tens place and the last instance of a numeric digit in the ones place.
/// Returns 0 if no numeric digits are in the String.
pub trait ICalibrateExt {
    type Error: std::error::Error + Send + Sync + 'static;
    fn calibrate(&self) -> Result<usize, Self::Error>;
}

impl ICalibrateExt for &str {
    type Error = Error;
    fn calibrate(&self) -> Result<usize> {
        let num_regex = Regex::new("one|two|three|four|five|six|seven|eight|nine|[1-9]")?;

        // Look for first match, first. If no matches found, early return 0.
        let tens_place = if let Some(first_match) = num_regex.find(self) {
            parse_calibration(first_match.as_str())?
        } else {
            return Ok(0);
        };

        // Look for second match by traversing backwards through the input, one index at a time.
        // Must use Regex::find_at because there is no Regex::rfind :(.
        let len = self.len();
        for i in 0..len {
            let index = len - i - 1;
            if let Some(last_match) = num_regex.find_at(self, index) {
                let ones_place = parse_calibration(last_match.as_str())?;
                let res = tens_place * 10 + ones_place;
                return Ok(res);
            }
        }
        Ok(0)
    }
}

fn parse_calibration(input: &str) -> Result<usize> {
    match input {
        "one" | "1" => Ok(1),
        "two" | "2" => Ok(2),
        "three" | "3" => Ok(3),
        "four" | "4" => Ok(4),
        "five" | "5" => Ok(5),
        "six" | "6" => Ok(6),
        "seven" | "7" => Ok(7),
        "eight" | "8" => Ok(8),
        "nine" | "9" => Ok(9),
        _ => Err(Error::InputCannotBeParsed(input.into())),
    }
}
