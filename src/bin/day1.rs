use std::usize;

use anyhow::Result;
use itertools::Itertools;

fn part_one() -> Result<usize> {
    Ok(aoc::read_one_per_line::<String>("./data/day1.input")?
        .iter()
        .filter_map(|line| {
            let digits = line
                .chars()
                .filter_map(|char| String::from(char).parse::<u32>().ok())
                .collect_vec();
            let mut num_string = String::new();
            if let Some(first_digit) = digits.first() {
                num_string.push(char::from_digit(*first_digit, 10).expect("unable to parse digit"));
            }
            if let Some(second_digit) = digits.last() {
                num_string
                    .push(char::from_digit(*second_digit, 10).expect("unable to parse digit"));
            }

            num_string.parse::<usize>().ok()
        })
        .sum())
}

fn main() -> Result<()> {
    println!("Part 1: {:?}", part_one());
    Ok(())
}
