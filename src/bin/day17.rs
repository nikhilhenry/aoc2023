#![feature(const_trait_impl)]
use std::collections::{BinaryHeap, HashMap, HashSet};

use anyhow::Result;
use aoc::{Grid, Position};
use lazy_static::lazy_static;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct Node {
    pos: Position,
    cost: usize,
    step_length: u8,
    dir: Direction,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct HeatBlock {
    grid: Grid<char>,
}

lazy_static! {
    static ref DIR_OFFSETS: HashMap<Direction, Position> = [
        (Direction::East, aoc::OFFSETS[0]),
        (Direction::West, aoc::OFFSETS[1]),
        (Direction::North, aoc::OFFSETS[2]),
        (Direction::South, aoc::OFFSETS[3]),
    ]
    .into();
}
impl HeatBlock {
    fn get_heat(&self, pos: &Position) -> usize {
        self.grid.get(pos).to_digit(10).unwrap() as usize
    }

    fn get_neighbours(&self, node: &Node) -> Vec<Node> {
        let mut offsets = DIR_OFFSETS.clone();
        // removing the reverse direction
        match node.dir {
            Direction::North => offsets.remove(&Direction::South),
            Direction::South => offsets.remove(&Direction::North),
            Direction::East => offsets.remove(&Direction::West),
            Direction::West => offsets.remove(&Direction::East),
        };

        if node.step_length == 3 {
            offsets.remove(&node.dir);
        }

        offsets
            .drain()
            .filter(|(_, pos)| self.grid.is_valid_pos(&(node.pos + pos)))
            .map(|(dir, off)| Node {
                step_length: if node.dir == dir {
                    node.step_length + 1
                } else {
                    1
                },
                cost: node.cost + self.get_heat(&(off + &node.pos)),
                pos: off + &node.pos,
                dir,
            })
            .collect()
    }

    fn find_path(&self) {
        let mut min_heap = BinaryHeap::new();
        let mut visited = HashSet::new();

        let start_node = Node {
            pos: aoc::pos!(0, 0),
            cost: 0,
            step_length: 1,
            dir: Direction::East,
        };

        let dest_pos = aoc::pos!(self.grid.rows - 1, self.grid.cols - 1);
        min_heap.push(start_node);

        while let Some(node) = min_heap.pop() {
            if visited.contains(&node) {
                continue;
            }
            if node.pos == dest_pos {
                println!("Part 1: {}", node.cost);
                break;
            }

            for neighbour in self.get_neighbours(&node) {
                min_heap.push(neighbour)
            }

            visited.insert(node);
        }
    }
}

fn main() -> Result<()> {
    let grid: Grid<char> = include_str!("../../data/day17.input").parse()?;
    let heat_block = HeatBlock { grid };
    heat_block.find_path();

    Ok(())
}
