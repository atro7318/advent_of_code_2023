pub mod error;

use error::{Error, Result};
use regex::Regex;

#[derive(Clone, Copy, Debug, Default)]
pub struct Grab {
    red_count: usize,
    blue_count: usize,
    green_count: usize,
}

impl Grab {
    pub fn new(red_count: usize, blue_count: usize, green_count: usize) -> Self {
        Self {
            red_count,
            blue_count,
            green_count,
        }
    }

    pub fn red_count(&self) -> usize {
        self.red_count
    }

    pub fn blue_count(&self) -> usize {
        self.blue_count
    }

    pub fn green_count(&self) -> usize {
        self.green_count
    }

    pub fn is_possible(&self, max_grab: Grab) -> bool {
        self.red_count <= max_grab.red_count
            && self.blue_count <= max_grab.blue_count
            && self.green_count <= max_grab.green_count
    }

    pub fn parse(s: &str) -> Result<Self> {
        let red_count = find_count(Regex::new(r"[0-9]+ red")?, s)?;
        let blue_count = find_count(Regex::new(r"[0-9]+ blue")?, s)?;
        let green_count = find_count(Regex::new(r"[0-9]+ green")?, s)?;

        let res = Self {
            red_count,
            blue_count,
            green_count,
        };

        Ok(res)
    }
}

fn find_count(regex: Regex, haystack: &str) -> Result<usize> {
    let res = regex.find(haystack).map_or_else(
        || Ok::<_, Error>(0),
        |m| {
            Regex::new(r"[0-9]+")
                .map_err(Into::<Error>::into)
                .map(|regex| regex.find(m.as_str()).ok_or_else(|| Error::ParseCount))?
                .map(|m2| m2.as_str().parse::<usize>().map_err(Into::<Error>::into))?
        },
    )?;
    Ok(res)
}
