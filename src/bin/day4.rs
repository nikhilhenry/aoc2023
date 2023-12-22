use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{anyhow, Error, Result};

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    winning_nums: HashSet<usize>,
    nums: Vec<usize>,
}

#[derive(Clone)]
struct CopyCard {
    copy_idxs: Option<Vec<usize>>,
}

impl Card {
    fn matching_count(&self) -> usize {
        self.nums
            .iter()
            .filter(|num| self.winning_nums.contains(num))
            .count()
    }
    fn points(&self) -> usize {
        let num_winning = self.matching_count();
        if num_winning > 0 {
            return 2_usize.pow((num_winning - 1) as u32);
        }
        0
    }
    fn matching_idxs(&self) -> Option<Vec<usize>> {
        if self.matching_count() == 0 {
            return None;
        }
        Some((self.id + 1..=self.id + self.matching_count()).collect())
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

    let copy_cards: HashMap<usize, CopyCard> = cards
        .iter()
        .map(|card| {
            (
                card.id,
                CopyCard {
                    copy_idxs: card.matching_idxs(),
                },
            )
        })
        .collect();

    let mut pile: Vec<CopyCard> = copy_cards.clone().into_values().collect();
    let mut idx = 0_usize;

    while idx < pile.len() {
        if let Some(idxs) = pile[idx].copy_idxs.clone() {
            idxs.iter()
                .for_each(|idx| pile.push(copy_cards.get(idx).unwrap().clone()));
        }
        idx += 1
    }

    println!("Part 2: {:?}", pile.len());

    Ok(())
}
