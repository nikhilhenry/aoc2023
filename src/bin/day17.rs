use std::collections::{BinaryHeap, HashSet};

use anyhow::Result;
use aoc::{Grid, Position};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
enum Direction {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
}

fn variant_eq(a: &Direction, b: &Direction) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct Node {
    pos: Position,
    running_cost: usize,
    h_cost: usize, // manhattan distance between it and dest
    dir: Direction,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let f_cost = other.h_cost + other.running_cost;
        f_cost.cmp(&(self.h_cost + self.running_cost))
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

impl HeatBlock {
    fn get_heat(&self, pos: &Position) -> usize {
        self.grid.get(pos).to_digit(10).unwrap() as usize
    }

    fn get_neighbours(&self, node: &Node) -> Vec<(Position, Direction)> {
        let mut raw_vec = aoc::OFFSETS.to_vec(); // right left top bottom
                                                 // delete reverse dir
        match node.dir {
            Direction::North(count) => {
                if count == 3 {
                    raw_vec.remove(2); // removing north
                    raw_vec.remove(2); // removing the new south
                } else {
                    raw_vec.remove(3); // removing south
                }
            }
            Direction::South(count) => {
                if count == 3 {
                    raw_vec.remove(3); // removing south
                    raw_vec.remove(2); // removing the new north
                } else {
                    raw_vec.remove(2); // removing north
                }
            }
            Direction::East(count) => {
                if count == 3 {
                    raw_vec.remove(0); // removing east
                    raw_vec.remove(0); // removing the new west
                } else {
                    raw_vec.remove(1); // removing west
                }
            }
            Direction::West(count) => {
                if count == 3 {
                    raw_vec.remove(1); // removing west
                    raw_vec.remove(0); // removing the new east
                } else {
                    raw_vec.remove(0); // removing west
                }
            }
        }

        raw_vec
            .into_iter()
            .filter(|pos| self.grid.is_valid_pos(&(pos + &node.pos)))
            .map(|pos| {
                // finding the direction
                let dir_idx = aoc::OFFSETS
                    .iter()
                    .position(|offset| offset == &pos)
                    .unwrap();
                let mut dir = match dir_idx {
                    0 => Direction::East(1),
                    1 => Direction::West(1),
                    2 => Direction::North(1),
                    3 => Direction::South(1),
                    _ => panic!("how did we get here?"),
                };

                if variant_eq(&dir, &node.dir) {
                    dir = match node.dir {
                        Direction::North(count) => Direction::North(count + 1),
                        Direction::South(count) => Direction::South(count + 1),
                        Direction::East(count) => Direction::East(count + 1),
                        Direction::West(count) => Direction::West(count + 1),
                    }
                }

                (pos + &node.pos, dir)
            })
            .collect()
    }

    fn path_find(&self) {
        let dest_pos = aoc::pos!(self.grid.rows - 1, self.grid.cols - 1);
        let start_pos = aoc::pos!(0, 0);
        let mut open_set: BinaryHeap<Node> = BinaryHeap::new();
        // add the starting node in east and south direction
        let n1 = Node {
            pos: start_pos.clone(),
            h_cost: aoc::manhattan_distance(&start_pos, &dest_pos),
            running_cost: 0,
            dir: Direction::East(1),
        };
        let n2 = Node {
            h_cost: aoc::manhattan_distance(&start_pos, &dest_pos),
            pos: start_pos,
            running_cost: 0,
            dir: Direction::South(1),
        };
        open_set.push(n1);
        //open_set.push(n2);

        let mut iter = 0;
        while let Some(node) = open_set.pop() {
            if iter >= 20 {
                //break;
            }
            println!("{:?}", node);
            if node.pos == dest_pos {
                println!("Part 1: {}", node.running_cost);
                todo!();
            }
            let neigbours = self.get_neighbours(&node);
            let neigbours = neigbours.into_iter().map(|(pos, dir)| {
                let node_cost = self.get_heat(&pos);
                Node {
                    h_cost: aoc::manhattan_distance(&pos, &dest_pos),
                    pos,
                    running_cost: node.running_cost + node_cost,
                    dir,
                }
            });
            neigbours.for_each(|neigbour| open_set.push(neigbour));
            //println!("{:?}", open_set);
            iter += 1;
        }
    }
}

fn main() -> Result<()> {
    let grid: Grid<char> = include_str!("../../data/day17.example").parse()?;
    let heat_block = HeatBlock { grid };
    //println!( "{:?}", heat_block.get_neighbours(&Node { pos: aoc::pos!(1, 1), running_cost: 0, h_cost: 0, dir: Direction::West(3) }));
    heat_block.path_find();

    Ok(())
}
