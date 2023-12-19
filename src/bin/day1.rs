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

fn parse_number(number: &str) -> Option<u32> {
    match number {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn part_two() -> Result<usize> {
    let patterns = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    let ac = aho_corasick::AhoCorasick::new(patterns).unwrap();
    Ok(aoc::read_one_per_line::<String>("./data/day1.input")?
        .iter()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            let nums = ac
                .find_overlapping_iter(line)
                .map(|m| String::from(&line[m.span().range()]))
                .collect_vec();
            let mut num_string = String::new();
            if let Some(first_number) = nums.first() {
                num_string.push(
                    char::from_digit(
                        parse_number(first_number).unwrap_or_else(|| {
                            first_number.parse::<u32>().expect("failed to parse number")
                        }),
                        10,
                    )
                    .unwrap(),
                );
            }
            if let Some(second_number) = nums.last() {
                num_string.push(
                    char::from_digit(
                        parse_number(second_number).unwrap_or_else(|| {
                            second_number
                                .parse::<u32>()
                                .expect("failed to parse number")
                        }),
                        10,
                    )
                    .unwrap(),
                );
            }
            num_string.parse::<usize>().ok()
        })
        .sum())
}

fn main() -> Result<()> {
    println!("Part 1: {:?}", part_one());
    println!("Part 2: {:?}", part_two());
    Ok(())
}
