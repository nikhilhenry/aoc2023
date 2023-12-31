use std::collections::{HashSet, VecDeque};

use anyhow::Result;
use aoc::{Grid, Position};

#[derive(Clone, Debug, Default)]
enum Node {
    #[default]
    Empty,
    RightMirror,
    LeftMirror,
    VSplitter,
    HSplitter,
}

#[derive(Hash, PartialEq, Clone, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn get_offset(&self) -> Position {
        match self {
            Direction::North => aoc::pos!(-1, 0),
            Direction::South => aoc::pos!(1, 0),
            Direction::West => aoc::pos!(0, -1),
            Direction::East => aoc::pos!(0, 1),
        }
    }
}

impl From<char> for Node {
    fn from(value: char) -> Self {
        match value {
            '/' => Node::RightMirror,
            '\\' => Node::LeftMirror,
            '-' => Node::HSplitter,
            '|' => Node::VSplitter,
            _ => Node::Empty,
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct VecPos {
    pos: Position,
    dir: Direction,
}

#[derive(Clone)]
struct Contraption {
    queue: VecDeque<VecPos>,
    grid: Grid<Node>,
    visited: HashSet<VecPos>,
}

fn reflect(mirror: &Node, dir: &Direction) -> Direction {
    match dir {
        Direction::North => match mirror {
            Node::RightMirror => Direction::East,
            Node::LeftMirror => Direction::West,
            _ => todo!(),
        },
        Direction::South => match mirror {
            Node::RightMirror => Direction::West,
            Node::LeftMirror => Direction::East,
            _ => todo!(),
        },

        Direction::West => match mirror {
            Node::RightMirror => Direction::South,
            Node::LeftMirror => Direction::North,
            _ => todo!(),
        },

        Direction::East => match mirror {
            Node::RightMirror => Direction::North,
            Node::LeftMirror => Direction::South,
            _ => todo!(),
        },
    }
}

impl Contraption {
    fn shine(&mut self, pos: VecPos) {
        if !self.grid.is_valid_pos(&pos.pos) {
            return;
        }
        let dir = pos.dir.clone();
        let poses = match self.grid.get(&pos.pos) {
            Node::Empty => vec![VecPos {
                pos: pos.pos.clone() + &pos.dir.get_offset(),
                dir: dir,
            }],
            Node::RightMirror => vec![VecPos {
                pos: pos.pos.clone() + &reflect(&Node::RightMirror, &pos.dir).get_offset(),
                dir: reflect(&Node::RightMirror, &pos.dir),
            }],
            Node::LeftMirror => vec![VecPos {
                pos: pos.pos.clone() + &reflect(&Node::LeftMirror, &pos.dir).get_offset(),
                dir: reflect(&Node::LeftMirror, &pos.dir),
            }],
            Node::VSplitter => match pos.dir {
                Direction::North => vec![VecPos {
                    pos: pos.pos.clone() + &pos.dir.get_offset(),
                    dir: dir,
                }],
                Direction::South => vec![VecPos {
                    pos: pos.pos.clone() + &pos.dir.get_offset(),
                    dir: dir,
                }],
                _ => vec![
                    VecPos {
                        pos: pos.pos.clone() + &Direction::North.get_offset(),
                        dir: Direction::North,
                    },
                    VecPos {
                        pos: pos.pos.clone() + &Direction::South.get_offset(),
                        dir: Direction::South,
                    },
                ],
            },
            Node::HSplitter => match pos.dir {
                Direction::East => vec![VecPos {
                    pos: pos.pos.clone() + &pos.dir.get_offset(),
                    dir: dir,
                }],
                Direction::West => vec![VecPos {
                    pos: pos.pos.clone() + &pos.dir.get_offset(),
                    dir: dir,
                }],
                _ => vec![
                    VecPos {
                        pos: pos.pos.clone() + &Direction::East.get_offset(),
                        dir: Direction::East,
                    },
                    VecPos {
                        pos: pos.pos.clone() + &Direction::West.get_offset(),
                        dir: Direction::West,
                    },
                ],
            },
        };
        self.visited.insert(pos);
        poses
            .into_iter()
            .filter(|posi| !self.visited.contains(&posi))
            .for_each(|posi| self.queue.push_back(posi));
    }

    fn run(&mut self) {
        while let Some(pos) = self.queue.pop_front() {
            self.shine(pos)
        }
    }

    fn energized(&self) -> usize {
        self.visited
            .iter()
            .map(|pos| pos.pos.clone())
            .collect::<HashSet<Position>>()
            .len()
    }
}

fn main() -> Result<()> {
    let grid: Grid<Node> = include_str!("../../data/day16.input").parse()?;
    let cols = grid.cols;
    let rows = grid.rows;

    let mut contraption = Contraption {
        grid,
        queue: VecDeque::new(),
        visited: HashSet::new(),
    };
    let copy_contrap = contraption.clone();
    contraption.shine(VecPos {
        pos: aoc::pos!(0, 0),
        dir: Direction::East,
    });

    contraption.run();

    println!("Part 1: {}", contraption.energized());

    // Part 2 - Shinangains
    // computing all the starting ray poses
    let top = (0..cols).map(|col| VecPos {
        pos: aoc::pos!(0, col),
        dir: Direction::South,
    });
    let bottom = (0..cols).map(|col| VecPos {
        pos: aoc::pos!(rows - 1, col),
        dir: Direction::North,
    });
    let left = (0..rows).map(|row| VecPos {
        pos: aoc::pos!(row, 0),
        dir: Direction::East,
    });
    let right = (0..rows).map(|row| VecPos {
        pos: aoc::pos!(row, cols - 1),
        dir: Direction::West,
    });

    let mut rays = Vec::new();
    rays.extend(top);
    rays.extend(bottom);
    rays.extend(left);
    rays.extend(right);

    let max_score = rays
        .into_iter()
        .map(|ray| {
            let mut contrap = copy_contrap.clone();
            contrap.shine(ray);
            contrap.run();
            contrap.energized()
        })
        .max()
        .unwrap();

    println!("Part 2: {max_score}");

    Ok(())
}
