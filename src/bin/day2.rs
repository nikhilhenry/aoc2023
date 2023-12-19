use anyhow::{anyhow, Error, Result};
use itertools::Itertools;
use std::{str::FromStr, u8, usize};

#[derive(Debug)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

impl FromStr for Set {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        for grp in s.trim().split(",") {
            if let Some((num, color)) = grp.trim().split_once(" ") {
                let color = color.trim();
                match color {
                    "red" => red = num.parse::<usize>().expect("failed to parse number"),
                    "blue" => blue = num.parse::<usize>().expect("failed to parse number"),
                    "green" => green = num.parse::<usize>().expect("failed to parse number"),
                    _ => panic!("invalid color:{}", color),
                }
            } else {
                return Err(anyhow!("failed to parse set"));
            }
        }
        Ok(Set { red, blue, green })
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((meta, raw_sets)) = s.split_once(":") else {
            return Err(anyhow!("game id not found"));
        };
        let Some((_, id)) = meta.split_once(" ") else {
            return Err(anyhow!("game id not found"));
        };
        let id = id.parse::<usize>()?;
        let sets = raw_sets
            .trim()
            .split(";")
            .filter_map(|set| set.parse::<Set>().ok())
            .collect_vec();

        Ok(Game { id, sets })
    }
}

impl Set {
    fn possible(&self, other: &Set) -> bool {
        self.red <= other.red && self.blue <= other.blue && self.green <= other.green
    }
}

impl Game {
    fn possible(&self, config: &Set) -> bool {
        self.sets
            .iter()
            .map(|set| if set.possible(config) { 1 } else { 0 })
            .product::<u8>()
            == 1
    }

    fn min_set(&self) -> Set {
        let max_red = self.sets.iter().map(|set| set.red).max().unwrap();
        let max_green = self.sets.iter().map(|set| set.green).max().unwrap();
        let max_blue = self.sets.iter().map(|set| set.blue).max().unwrap();

        Set {
            red: max_red,
            green: max_green,
            blue: max_blue,
        }
    }
    fn minimum_power(&self) -> usize {
        let min_set = self.min_set();
        min_set.red * min_set.blue * min_set.green
    }
}

fn main() -> Result<()> {
    let data = aoc::read_one_per_line::<Game>("./data/day2.input")?;
    let config = Set {
        red: 12,
        green: 13,
        blue: 14,
    };
    println!(
        "Part 1: {:?}",
        data.iter()
            .filter(|game| game.possible(&config))
            .map(|game| game.id)
            .sum::<usize>()
    );
    println!(
        "Part 2: {:?}",
        data.iter().map(|game| game.minimum_power()).sum::<usize>()
    );
    Ok(())
}
