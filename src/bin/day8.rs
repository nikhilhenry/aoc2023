use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Result};

#[derive(Debug)]
enum Instruction {
    Right,
    Left,
}

#[derive(Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (id, contents) = s.split_once(" = ").ok_or(anyhow!("invalid string"))?;
        let (left, right) = contents.split_once(",").unwrap();

        let mut left = String::from(left);
        left.remove(0);

        let mut right = String::from(right);
        right.remove(0);
        right.pop();

        Ok(Self {
            id: String::from(id),
            left,
            right,
        })
    }
}

impl Node {
    fn process(&self, ins: &Instruction) -> String {
        match ins {
            Instruction::Right => self.right.to_string(),
            Instruction::Left => self.left.to_string(),
        }
    }
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    s.chars()
        .map(|c| match c {
            'R' => Instruction::Right,
            'L' => Instruction::Left,
            _ => panic!("invalid char"),
        })
        .collect()
}

fn main() -> Result<()> {
    let s = include_str!("../../data/day8.input");
    let (instructions, s) = s.split_once("\n\n").unwrap();
    let instructions = parse_instructions(instructions);

    let network: HashMap<String, Node> = s
        .split("\n")
        .filter_map(|s| s.parse::<Node>().ok())
        .map(|node| (node.id.to_string(), node))
        .collect();

    let mut step = 0;
    let mut cursor = String::from("AAA");
    let mag = instructions.len();

    while cursor != "ZZZ" {
        cursor = network
            .get(&cursor)
            .unwrap_or_else(|| panic!("invalid node {cursor}"))
            .process(&instructions[step % mag]);
        step += 1;
    }

    println!("steps: {step}");
    Ok(())
}
