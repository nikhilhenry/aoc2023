#![allow(unused)]
use std::{cmp::Ordering, i32, str::FromStr};

use anyhow::{anyhow, Result};
use aoc::{pos, Direction, Position};
use itertools::Itertools;

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    mag: usize,
    color: String,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (dir, s) = s.split_once(" ").ok_or(anyhow!("Failed to parse"))?;
        let dir = parse_dir(dir)?;
        let (num, s) = s.split_once(" ").ok_or(anyhow!("Failed to parse"))?;
        let mag = num.parse()?;
        Ok(Self {
            dir,
            mag,
            color: s.to_string(),
        })
    }
}

fn parse_dir(s: &str) -> Result<Direction> {
    match s {
        "R" => Ok(Direction::East),
        "L" => Ok(Direction::West),
        "U" => Ok(Direction::North),
        "D" => Ok(Direction::South),
        _ => Err(anyhow!("invalid direction")),
    }
}

fn sort_cc_points(a: &Position, _: &Position) -> Ordering {
    let ang_1 = f64::atan2(a.row.into(), a.col.into());

    ang_1.total_cmp(&0.0)
}

fn main() -> Result<()> {
    let data = aoc::read_one_per_line::<Instruction>("./data/day18.input")?;
    let mut cursor = aoc::pos!(0, 0);
    let mut points = vec![cursor.clone()];

    let mut perimeter = 0;
    for instruction in data {
        for _ in 0..instruction.mag {
            cursor = cursor + &aoc::DIR_OFFSETS[&instruction.dir];
            perimeter += 1
        }
        points.push(cursor.clone())
    }

    let mut sum = 0.0_f64;
    for idx in 0..points.len() - 1 {
        sum += (points[idx].col * points[idx + 1].row) as f64;
        sum -= (points[idx + 1].col * points[idx].row) as f64
    }

    let area = (sum.abs() as i32 / 2) + (perimeter / 2) + 1;
    println!("Part 1:{area}");

    Ok(())
}
