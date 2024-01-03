use std::str::FromStr;

use anyhow::{anyhow, Result};
use aoc::{Direction, Position};

trait Instruction {
    fn dir(&self) -> &Direction;
    fn mag(&self) -> usize;
}

#[derive(Debug)]
struct Instruction1 {
    dir: Direction,
    mag: usize,
}

impl Instruction for Instruction1 {
    fn dir(&self) -> &Direction {
        &self.dir
    }

    fn mag(&self) -> usize {
        self.mag
    }
}

#[derive(Debug)]
struct Instruction2 {
    dir: Direction,
    mag: usize,
}

impl Instruction for Instruction2 {
    fn dir(&self) -> &Direction {
        &self.dir
    }

    fn mag(&self) -> usize {
        self.mag
    }
}

impl FromStr for Instruction1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (dir, s) = s.split_once(" ").ok_or(anyhow!("Failed to parse"))?;
        let dir = parse_dir(dir)?;
        let (num, _) = s.split_once(" ").ok_or(anyhow!("Failed to parse"))?;
        let mag = num.parse()?;
        Ok(Self { dir, mag })
    }
}

impl FromStr for Instruction2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (_, s) = s.split_once(" ").ok_or(anyhow!("Failed to parse"))?;
        let (_, s) = s.split_once(" ").ok_or(anyhow!("Failed to parse"))?;
        let mut s = s.to_string();
        s.pop();
        s.remove(0);
        s.remove(0);
        let dir = s.pop().unwrap();
        let dir = match dir {
            '0' => Ok(Direction::East),
            '1' => Ok(Direction::South),
            '2' => Ok(Direction::West),
            '3' => Ok(Direction::North),
            _ => Err(anyhow!("invalid direction")),
        }?;
        let mag = usize::from_str_radix(&s, 16)?;
        Ok(Instruction2 { dir, mag })
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

fn shoelace<T: Instruction>(instructions: Vec<T>) -> usize {
    let mut cursor = aoc::pos!(0, 0);
    let mut points = vec![cursor.clone()];

    let mut perimeter = 0;
    for instruction in instructions {
        for _ in 0..instruction.mag() {
            cursor = cursor + &aoc::DIR_OFFSETS[&instruction.dir()];
            perimeter += 1
        }
        points.push(cursor.clone())
    }

    let mut sum = 0.0_f64;
    for idx in 0..points.len() - 1 {
        sum += points[idx].col as f64 * points[idx + 1].row as f64;
        sum -= points[idx + 1].col as f64 * points[idx].row as f64;
    }

    (sum.abs() as usize / 2) + (perimeter / 2) + 1
}

fn main() -> Result<()> {
    let data = aoc::read_one_per_line::<Instruction1>("./data/day18.input")?;
    println!("Part 1:{}", shoelace(data));

    let data = aoc::read_one_per_line::<Instruction2>("./data/day18.input")?;
    println!("Part 2:{}", shoelace(data));

    Ok(())
}
