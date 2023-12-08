pub mod error;
mod symbol_coordinate;

use bool_ext::BoolExt;
use error::Result;
use regex::Regex;
use std::collections::HashMap;
use symbol_coordinate::SymbolCoordinate;

pub struct EngineSchematic(HashMap<SymbolCoordinate, Vec<u64>>);

impl EngineSchematic {
    pub fn new(input: String, symbol_finder: impl Fn(char) -> bool) -> Result<Self> {
        let mut gear_grid = Vec::new();
        for line in input.lines() {
            let row = line.chars().map(|c| symbol_finder(c)).collect::<Vec<_>>();
            gear_grid.push(row);
        }

        let mut inner = HashMap::<SymbolCoordinate, Vec<u64>>::new();

        let mut row = 0;
        let num_regex = Regex::new(r"[0-9]+")?;
        for line in input.lines() {
            for m in num_regex.find_iter(line) {
                let match_range = m.range();
                if let Some(gear_coordinate) =
                    Self::find_symbol(&gear_grid, row, match_range.start, match_range.end)
                {
                    let num = m.as_str().parse()?;
                    inner
                        .entry(gear_coordinate)
                        .and_modify(|list| list.push(num))
                        .or_insert(vec![num]);
                }
            }
            row += 1;
        }

        Ok(Self(inner))
    }

    fn find_symbol(
        symbol_grid: &Vec<Vec<bool>>,
        row: usize,
        col_start: usize,
        col_end: usize,
    ) -> Option<SymbolCoordinate> {
        let left_bound = col_start.saturating_sub(1);
        let right_bound = (col_end < symbol_grid[0].len() - 1)
            .map_or_else(|| symbol_grid[0].len() - 1, || col_end + 1);

        if row > 0 {
            for col_i in left_bound..right_bound {
                if symbol_grid[row - 1][col_i] {
                    return Some(SymbolCoordinate::new(row - 1, col_i));
                }
            }
        }

        if symbol_grid[row][left_bound] {
            return Some(SymbolCoordinate::new(row, left_bound));
        }

        if symbol_grid[row][right_bound - 1] {
            return Some(SymbolCoordinate::new(row, right_bound - 1));
        }

        if row < symbol_grid.len() - 1 {
            for col_i in left_bound..right_bound {
                if symbol_grid[row + 1][col_i] {
                    return Some(SymbolCoordinate::new(row + 1, col_i));
                }
            }
        }

        None
    }

    pub fn ratio_sum(&self) -> u64 {
        self.0
            .values()
            .map(|maybe_ratio| {
                (maybe_ratio.len() == 2).map_or_else(|| 0, || maybe_ratio[0] * maybe_ratio[1])
            })
            .fold(0, |acc, ratio| acc + ratio)
    }

    pub fn sum(&self) -> u64 {
        self.0
            .values()
            .flat_map(|v| v)
            .fold(0, |acc, ratio| acc + ratio)
    }
}
