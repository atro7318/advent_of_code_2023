pub mod error;
pub mod grab;

use std::cmp::max;
use crate::game::error::Error;
use error::Result;
use grab::Grab;
use regex::Regex;

pub struct Game {
    id: u32,
    grabs: Box<[Grab]>,
}

impl Game {
    pub fn parse(s: &str) -> Result<Self> {
        let colon_split = s.split(':').collect::<Box<[_]>>();
        let id = Regex::new(r"[0-9]+")?
            .find(colon_split[0])
            .ok_or_else(|| Error::ParseId)?
            .as_str()
            .parse()?;
        let grabs = colon_split[1]
            .split(";")
            .map(|grab_str| Grab::parse(grab_str).map_err(Into::into))
            .collect::<Result<Box<[_]>>>()?;
        let res = Self { id, grabs };
        Ok(res)
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn is_valid_game(&self, max_grab: Grab) -> bool {
        for grab in self.grabs.iter() {
            if !grab.is_possible(max_grab) {
                return false;
            }
        }
        true
    }

    pub fn min_valid_grab(&self) -> Grab {
        let mut max_red_count = 0;
        let mut max_blue_count = 0;
        let mut max_green_count = 0;

        self.grabs.iter().for_each(|grab| {
            max_red_count = max(max_red_count, grab.red_count());
            max_blue_count = max(max_blue_count, grab.blue_count());
            max_green_count = max(max_green_count, grab.green_count());
        });

        Grab::new(max_red_count, max_blue_count, max_green_count)
    }
}
