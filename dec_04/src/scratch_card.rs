pub mod error;

use bool_ext::BoolExt;
use error::Result;
use std::collections::HashSet;

pub struct ScratchCard {
    id: usize,
    winning_nums: HashSet<u8>,
    nums: Box<[u8]>,
}

impl ScratchCard {
    pub fn parse(s: &str) -> Result<Self> {
        let id_comma_num_groups = s.split(':').collect::<Box<[_]>>();
        let id = id_comma_num_groups[0]
            .strip_prefix("Card")
            .expect("Malformed line, cannot parse. Expected line to start with 'Card '")
            .trim()
            .parse()?;
        let num_groups = id_comma_num_groups[1].split('|').collect::<Box<[_]>>();
        let winning_nums = num_groups[0]
            .trim()
            .split_whitespace()
            .map(|n| n.parse().map_err(Into::into))
            .collect::<Result<HashSet<_>>>()?;
        let nums = num_groups[1]
            .trim()
            .split_whitespace()
            .map(|n| n.parse().map_err(Into::into))
            .collect::<Result<Box<[_]>>>()?;

        let res = Self {
            id,
            winning_nums,
            nums,
        };

        Ok(res)
    }

    pub fn calculate_score(&self) -> usize {
        self.nums.iter().fold(0, |acc, num| {
            self.winning_nums
                .contains(num)
                .map_or_else(|| acc, || (acc == 0).map_or_else(|| acc * 2, || 1))
        })
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn num_matches(&self) -> usize {
        self.nums.iter().fold(0, |acc, num| {
            acc + self.winning_nums.contains(num).map_or_else(|| 0, || 1)
        })
    }
}
