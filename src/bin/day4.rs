use std::{collections::HashSet, str::FromStr};

use anyhow::{anyhow, Error, Result};

#[derive(Debug)]
struct Card {
    id: usize,
    winning_nums: HashSet<usize>,
    nums: HashSet<usize>,
}

impl Card {
    fn points(&self) -> usize {
        let num_winning = self
            .nums
            .iter()
            .filter(|num| self.winning_nums.contains(num))
            .count();

        if num_winning > 0 {
            return 2_usize.pow((num_winning - 1) as u32);
        }
        0
    }
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (idx, lists) = s.split_once(":").ok_or(anyhow!("invalid input"))?;
        let (_, idx) = idx.split_once(" ").unwrap();
        let id = idx.trim().to_string().parse().expect("idx not valid ");

        let (winning_nums, nums) = lists.trim().split_once("|").unwrap();

        let winning_nums = winning_nums
            .split(" ")
            .filter_map(|num| num.parse::<usize>().ok())
            .collect();

        let nums = nums
            .split(" ")
            .filter_map(|num| num.parse::<usize>().ok())
            .collect();

        Ok(Card {
            id,
            winning_nums,
            nums,
        })
    }
}

fn main() -> Result<()> {
    let cards = aoc::read_one_per_line::<Card>("./data/day4.input")?;
    println!(
        "Part 1: {:?}",
        cards.iter().map(|card| card.points()).sum::<usize>()
    );
    Ok(())
}
