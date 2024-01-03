use std::collections::{BinaryHeap, HashSet};

use anyhow::Result;
use aoc::{Direction, Grid, Position};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct Node {
    pos: Position,
    heat: usize,
    dist: usize,
    step_length: u8,
    dir: Direction,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_cost = self.heat + self.dist;
        let other_cost = other.heat + other.dist;
        //other.heat.cmp(&self.heat)
        other_cost.cmp(&self_cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct HeatBlock {
    grid: Grid<char>,
    dest_pos: Position,
}

impl HeatBlock {
    fn get_heat(&self, pos: &Position) -> usize {
        self.grid.get(pos).to_digit(10).unwrap() as usize
    }

    fn get_neighbours(&self, node: &Node, min: u8, max: u8) -> Vec<Node> {
        let mut offsets = aoc::DIR_OFFSETS.clone();
        // removing the reverse direction
        match node.dir {
            Direction::North => offsets.remove(&Direction::South),
            Direction::South => offsets.remove(&Direction::North),
            Direction::East => offsets.remove(&Direction::West),
            Direction::West => offsets.remove(&Direction::East),
        };

        if node.step_length == max {
            offsets.remove(&node.dir);
        }

        if node.step_length < min {
            offsets = [(node.dir.clone(), offsets[&node.dir].clone())]
                .into_iter()
                .collect()
        }

        offsets
            .drain()
            .map(|(dir, off)| (dir, node.pos + &off))
            .filter(|(_, pos)| self.grid.is_valid_pos(pos))
            .map(|(dir, pos)| Node {
                step_length: if node.dir == dir {
                    node.step_length + 1
                } else {
                    1
                },
                heat: node.heat + self.get_heat(&pos),
                dist: aoc::manhattan_distance(&pos, &self.dest_pos),
                pos,
                dir,
            })
            .collect()
    }

    fn find_path(&self, min: u8, max: u8) -> usize {
        let mut min_heap = BinaryHeap::new();
        let mut visited = HashSet::new();

        let start_node = Node {
            pos: aoc::pos!(0, 0),
            heat: 0,
            dist: aoc::manhattan_distance(&aoc::pos!(0, 0), &self.dest_pos),
            step_length: 1,
            dir: Direction::East,
        };

        min_heap.push(start_node);

        while let Some(node) = min_heap.pop() {
            if visited.contains(&node) {
                continue;
            }
            if node.pos == self.dest_pos && node.step_length > min {
                return node.heat;
            }

            for neighbour in self.get_neighbours(&node, min, max) {
                min_heap.push(neighbour)
            }

            visited.insert(node);
        }

        panic!("Failed to find path")
    }
}

fn main() -> Result<()> {
    let grid: Grid<char> = include_str!("../../data/day17.input").parse()?;
    let dest_pos = aoc::pos!(grid.rows - 1, grid.cols - 1);
    let heat_block = HeatBlock { dest_pos, grid };
    println!("Part 1: {}", heat_block.find_path(0, 3));
    println!("Part 2: {}", heat_block.find_path(4, 10));

    Ok(())
}
