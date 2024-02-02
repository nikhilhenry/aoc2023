use std::{collections::HashMap, str::FromStr, usize};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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

fn traverse_network(
    mut cursor: String,
    num_zs: usize,
    network: &HashMap<String, Node>,
    instructions: &Vec<Instruction>,
) -> usize {
    let mut step = 0;
    let mag = instructions.len();
    let end: String = (0..num_zs).map(|_| 'Z').collect();

    while cursor.chars().rev().take(num_zs).collect::<String>() != end {
        cursor = network
            .get(&cursor)
            .unwrap_or_else(|| panic!("invalid node {cursor}"))
            .process(&instructions[step % mag]);
        step += 1;
    }
    step
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn traverse_network_parrallel(
    cursors: Vec<String>,
    network: &HashMap<String, Node>,
    instructions: &Vec<Instruction>,
) -> usize {
    let steps: Vec<usize> = cursors
        .par_iter()
        .map(|c| traverse_network(c.to_string(), 1, network, instructions))
        .collect();
    lcm(&steps)
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

    let step = traverse_network(String::from("AAA"), 3, &network, &instructions);
    println!("Part 1: {step}");

    // part 2
    let nodes = network.keys().collect_vec();
    let start_nodes = nodes
        .iter()
        .filter(|n| n.chars().last().unwrap() == 'A')
        .map(|s| s.to_string())
        .collect_vec();
    let step = traverse_network_parrallel(start_nodes, &network, &instructions);
    println!("Part 2: {step}");
    Ok(())
}
