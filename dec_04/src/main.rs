pub mod error;
mod scratch_card;

use crate::scratch_card::ScratchCard;
use error::Result;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;

const INPUT_FILE_PATH: &str = "/Users/avasurus/advent_of_code_2023/dec_04/input.txt";

fn main() -> Result<()> {
    // Part 1
    let mut total_score = 0;
    for line in read_to_string::<PathBuf>(INPUT_FILE_PATH.into())?.lines() {
        let scratch_card = ScratchCard::parse(&line)?;
        total_score += scratch_card.calculate_score();
    }
    println!("Total score: {total_score}");

    // Part 2
    let mut total_cards = 0;
    let mut id_to_copies = HashMap::<usize, usize>::new();
    for line in read_to_string::<PathBuf>(INPUT_FILE_PATH.into())?.lines() {
        let scratch_card = ScratchCard::parse(&line)?;

        let copies_count = id_to_copies
            .get(&scratch_card.id())
            .map_or_else(|| 1, |count| count + 1);

        (1..=scratch_card.num_matches()).for_each(|i| {
            id_to_copies
                .entry(i + scratch_card.id())
                .and_modify(|count| *count += copies_count)
                .or_insert(copies_count);
        });

        total_cards += copies_count;
    }
    println!("Total cards: {total_cards}");

    Ok(())
}
